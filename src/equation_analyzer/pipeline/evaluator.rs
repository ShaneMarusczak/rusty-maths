use crate::equation_analyzer::catalog::SymbolKind;
use crate::equation_analyzer::structs::token::{Token, TokenType};

/// Represents a function call frame for variadic functions
struct FunctionFrame {
    /// Position in the stack where this function's parameters start
    stack_position: usize,
}

/// A value on the evaluation stack.
///
/// `is_percent` is set by the postfix `%` operator: `+` and `-` treat a
/// percent-tagged right operand as relative to their left operand
/// (`100 - 20%` = 80, handheld-calculator style). Every other consumer
/// reads `num` literally — it has already been divided by 100.
#[derive(Clone, Copy)]
struct StackVal {
    num: f32,
    is_percent: bool,
}

fn plain(num: f32) -> StackVal {
    StackVal {
        num,
        is_percent: false,
    }
}

/// Pops the current function frame and splits off its parameters from the stack.
fn pop_frame(
    frames: &mut Vec<FunctionFrame>,
    stack: &mut Vec<StackVal>,
    name: &str,
) -> Result<Vec<f32>, String> {
    let frame = frames
        .pop()
        .ok_or_else(|| format!("Unexpected end of {name} call"))?;
    Ok(stack
        .split_off(frame.stack_position)
        .iter()
        .map(|v| v.num)
        .collect())
}

fn plural(n: u8) -> &'static str {
    if n == 1 {
        "parameter"
    } else {
        "parameters"
    }
}

/// Generic RPN evaluator that works with any iterator of tokens.
///
/// This is the core evaluation logic shared by all pipeline implementations.
/// It evaluates mathematical expressions in Reverse Polish Notation using a stack-based algorithm.
///
/// # Arguments
/// * `tokens` - An iterator of tokens in RPN format
/// * `x` - Optional value of the variable x (defaults to 0.0 if None)
///
/// # Returns
/// * `Ok(f32)` - The result of the evaluation
/// * `Err(String)` - An error message if evaluation fails
///
/// # Algorithm
/// 1. Maintains a value stack for intermediate results
/// 2. Uses frame markers to handle variadic functions (avg, min, max, etc.)
/// 3. Processes each token:
///    - Numbers/Constants: Push to stack
///    - Unary Calls and operators: Pop, apply (via the backing Symbol), push
///    - Binary operators: Pop twice, apply, push
///    - Variadic Calls: Mark stack position with a frame
///    - EndCall: Collect params since the frame position, arity-check,
///      dispatch through the Symbol, push result
/// 4. Returns final stack value (should be exactly 1 value)
pub(crate) fn evaluate<I>(tokens: I, x: impl Into<Option<f32>>) -> Result<f32, String>
where
    I: IntoIterator<Item = Token>,
{
    let x = x.into().unwrap_or(0.0);
    let mut stack: Vec<StackVal> = Vec::new();
    let mut frames: Vec<FunctionFrame> = Vec::new();
    let mut token_count = 0;

    for token in tokens {
        token_count += 1;

        match token.token_type {
            // Call: dispatches through the backing Symbol. Unary/UnaryChecked
            // pop one arg and push the result; Variadic starts a frame
            // (parameters get collected until the matching EndCall).
            TokenType::Call => {
                let sym = token
                    .symbol
                    .ok_or("Internal: Call token missing symbol")?;
                match sym.kind {
                    SymbolKind::Unary(f) => {
                        let v = stack.pop().ok_or_else(|| {
                            format!("Insufficient operands for {} function", sym.name)
                        })?;
                        stack.push(plain(f(v.num)));
                    }
                    SymbolKind::UnaryChecked(f) => {
                        let v = stack.pop().ok_or_else(|| {
                            format!("Insufficient operands for {} function", sym.name)
                        })?;
                        stack.push(plain(f(v.num)?));
                    }
                    SymbolKind::Variadic { .. } => {
                        frames.push(FunctionFrame {
                            stack_position: stack.len(),
                        });
                    }
                    _ => {
                        return Err(format!(
                            "Non-callable symbol '{}' at Call token",
                            sym.name
                        ));
                    }
                }
            }
            // EndCall: pop the frame, arity-check, dispatch the variadic.
            TokenType::EndCall => {
                let sym = token
                    .symbol
                    .ok_or("Internal: EndCall token missing symbol")?;
                let params = pop_frame(&mut frames, &mut stack, sym.name)?;
                match sym.kind {
                    SymbolKind::Variadic {
                        min_args,
                        max_args,
                        run,
                    } => {
                        let n = params.len();
                        if (n as u32) < min_args as u32 {
                            return Err(format!(
                                "{} requires at least {} {}, got {}",
                                sym.name,
                                min_args,
                                plural(min_args),
                                n
                            ));
                        }
                        if let Some(max) = max_args {
                            if (n as u32) > max as u32 {
                                return Err(format!(
                                    "{} accepts at most {} {}, got {}",
                                    sym.name,
                                    max,
                                    plural(max),
                                    n
                                ));
                            }
                        }
                        stack.push(plain(run(&params)?));
                    }
                    _ => {
                        return Err(format!(
                            "EndCall for non-variadic symbol '{}'",
                            sym.name
                        ));
                    }
                }
            }
            // Named constants (π, e, ...): value comes from the Symbol.
            TokenType::Constant => {
                let sym = token
                    .symbol
                    .ok_or("Internal: Constant token missing symbol")?;
                if let SymbolKind::Constant(v) = sym.kind {
                    stack.push(plain(v));
                } else {
                    return Err(format!(
                        "Constant token for non-constant symbol '{}'",
                        sym.name
                    ));
                }
            }
            TokenType::Number => stack.push(plain(token.numeric_value_1)),
            TokenType::UnaryMinus => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for unary minus operator")?;
                stack.push(plain(-temp.num));
            }
            TokenType::Factorial => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for factorial operator")?
                    .num;
                if temp < 0.0 || temp % 1.0 != 0.0 {
                    return Err("Factorial is only defined for non-negative integers".to_string());
                }
                stack.push(plain(crate::utilities::factorial(temp as isize) as f32));
            }
            // Postfix `%`: divide by 100 and tag the result so a following
            // `+`/`-` can scale it against the left operand (handheld
            // calculator semantics: 100 - 20% = 80, 200 + 10% = 220).
            // Every other operator consumes the tag and produces a plain
            // value — `%` is one-shot, not sticky.
            TokenType::Percent => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for percent operator")?;
                stack.push(StackVal {
                    num: temp.num / 100.0,
                    is_percent: true,
                });
            }
            TokenType::Log => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for log function")?
                    .num;
                stack.push(plain(temp.log(token.numeric_value_1)));
            }
            TokenType::X => stack.push(plain(
                token.numeric_value_1 * x.powf(token.numeric_value_2),
            )),
            _ => {
                if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
                    let result = match token.token_type {
                        TokenType::Plus => {
                            if rhs.is_percent {
                                lhs.num + lhs.num * rhs.num
                            } else {
                                lhs.num + rhs.num
                            }
                        }
                        TokenType::Minus => {
                            if rhs.is_percent {
                                lhs.num - lhs.num * rhs.num
                            } else {
                                lhs.num - rhs.num
                            }
                        }
                        TokenType::Star => lhs.num * rhs.num,
                        TokenType::Slash => lhs.num / rhs.num,
                        TokenType::Modulo => lhs.num % rhs.num,
                        TokenType::Power => lhs.num.powf(rhs.num),
                        _ => return Err(format!("Unknown token: {:?}", token)),
                    };
                    stack.push(plain(result));
                } else {
                    return Err("Invalid expression".to_string());
                }
            }
        }
    }

    if token_count == 0 {
        return Err(String::from("Invalid equation supplied"));
    }

    if stack.len() != 1 {
        return Err(format!(
            "Invalid evaluation: expected 1 result, found {} items in stack",
            stack.len()
        ));
    }

    stack
        .pop()
        .map(|v| v.num)
        .ok_or_else(|| "Evaluation stack is empty".to_string())
}
