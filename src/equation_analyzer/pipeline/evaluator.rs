use crate::equation_analyzer::catalog::SymbolKind;
use crate::equation_analyzer::errors::EquationError;
use crate::equation_analyzer::structs::token::{SpannedToken, Token};
use crate::utilities::factorial;

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
/// * `tokens` - An iterator of spanned tokens in RPN format
/// * `x` - Optional value of the variable x (defaults to 0.0 if None)
///
/// # Returns
/// * `Ok(f32)` - The result of the evaluation
/// * `Err(EquationError)` - An error; runtime failures point at the token
///   (or, for framed calls, the whole call) that caused them
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
pub(crate) fn evaluate<I>(tokens: I, x: impl Into<Option<f32>>) -> Result<f32, EquationError>
where
    I: IntoIterator<Item = SpannedToken>,
{
    let x = x.into().unwrap_or(0.0);
    let mut stack: Vec<StackVal> = Vec::new();
    let mut frames: Vec<FunctionFrame> = Vec::new();
    let mut token_count = 0;

    for spanned in tokens {
        token_count += 1;
        let token = spanned.token;
        // Attach the current token's span to an error message.
        let fail = |message: String| EquationError::spanned(message, spanned.span);

        match token {
            // Call: a pipe target (`x |> sin`) — the sole argument is
            // already on the stack. Parenthesized calls never produce this;
            // they arrive as CallStart…EndCall frames.
            Token::Call(sym) => match sym.kind {
                SymbolKind::Unary(f) => {
                    let v = stack.pop().ok_or_else(|| {
                        fail(format!("Insufficient operands for {} function", sym.name))
                    })?;
                    stack.push(plain(f(v.num)));
                }
                SymbolKind::UnaryChecked(f) => {
                    let v = stack.pop().ok_or_else(|| {
                        fail(format!("Insufficient operands for {} function", sym.name))
                    })?;
                    stack.push(plain(f(v.num).map_err(fail)?));
                }
                _ => {
                    return Err(fail(format!(
                        "Non-callable symbol '{}' at Call token",
                        sym.name
                    )));
                }
            },
            // CallStart: a parenthesized call opens a frame; its arguments
            // collect on the stack until the matching EndCall.
            Token::CallStart(_) => {
                frames.push(FunctionFrame {
                    stack_position: stack.len(),
                });
            }
            // EndCall: close the frame, enforce the catalog's arity, and
            // dispatch. Its span covers the whole call (`ch(25, 2)`), so
            // every error here underlines the full call site.
            Token::EndCall(sym) => {
                let frame = frames
                    .pop()
                    .ok_or_else(|| fail(format!("Unexpected end of {} call", sym.name)))?;
                let n = stack.len().saturating_sub(frame.stack_position);

                let (min_args, max_args) = match sym.kind {
                    SymbolKind::Unary(_) | SymbolKind::UnaryChecked(_) => (1, Some(1)),
                    SymbolKind::Variadic {
                        min_args, max_args, ..
                    } => (min_args, max_args),
                    _ => {
                        return Err(fail(format!(
                            "EndCall for non-callable symbol '{}'",
                            sym.name
                        )));
                    }
                };

                if (n as u32) < min_args as u32 {
                    return Err(fail(format!(
                        "{} requires at least {} {}, got {}",
                        sym.name,
                        min_args,
                        plural(min_args),
                        n
                    )));
                }
                if let Some(max) = max_args {
                    if (n as u32) > max as u32 {
                        return Err(fail(format!(
                            "{} accepts at most {} {}, got {}",
                            sym.name,
                            max,
                            plural(max),
                            n
                        )));
                    }
                }

                let result = match sym.kind {
                    // Arity is exactly 1 here, so dispatch straight off the
                    // stack top — no argument buffer needed.
                    SymbolKind::Unary(f) => stack.pop().map(|v| f(v.num)),
                    SymbolKind::UnaryChecked(f) => match stack.pop() {
                        Some(v) => Some(f(v.num).map_err(fail)?),
                        None => None,
                    },
                    SymbolKind::Variadic { run, .. } => {
                        let params: Vec<f32> = stack
                            .split_off(frame.stack_position)
                            .iter()
                            .map(|v| v.num)
                            .collect();
                        Some(run(&params).map_err(fail)?)
                    }
                    // Excluded by the arity match above.
                    _ => None,
                };
                let result = result
                    .ok_or_else(|| fail(format!("Insufficient operands for {}", sym.name)))?;
                stack.push(plain(result));
            }
            // Named constants (π, e, ...): value comes from the Symbol.
            Token::Constant(sym) => {
                if let SymbolKind::Constant(v) = sym.kind {
                    stack.push(plain(v));
                } else {
                    return Err(fail(format!(
                        "Constant token for non-constant symbol '{}'",
                        sym.name
                    )));
                }
            }
            Token::Number(n) => stack.push(plain(n)),
            Token::X => stack.push(plain(x)),
            Token::UnaryMinus => {
                let temp = stack
                    .pop()
                    .ok_or_else(|| fail("Insufficient operands for unary minus operator".into()))?;
                stack.push(plain(-temp.num));
            }
            Token::Factorial => {
                let temp = stack
                    .pop()
                    .ok_or_else(|| fail("Insufficient operands for factorial operator".into()))?
                    .num;
                if temp < 0.0 || temp % 1.0 != 0.0 {
                    return Err(fail(
                        "Factorial is only defined for non-negative integers".into(),
                    ));
                }
                stack.push(plain(factorial(temp as isize).map_err(fail)? as f32));
            }
            // Postfix `%`: divide by 100 and tag the result so a following
            // `+`/`-` can scale it against the left operand (handheld
            // calculator semantics: 100 - 20% = 80, 200 + 10% = 220).
            // Every other operator consumes the tag and produces a plain
            // value — `%` is one-shot, not sticky.
            Token::Percent => {
                let temp = stack
                    .pop()
                    .ok_or_else(|| fail("Insufficient operands for percent operator".into()))?;
                stack.push(StackVal {
                    num: temp.num / 100.0,
                    is_percent: true,
                });
            }
            Token::Log { base } => {
                let temp = stack
                    .pop()
                    .ok_or_else(|| fail("Insufficient operands for log function".into()))?
                    .num;
                stack.push(plain(temp.log(base)));
            }
            // Binary operators: pop rhs then lhs, apply, push.
            Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Modulo
            | Token::Power => {
                let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) else {
                    return Err(fail("Invalid expression".into()));
                };
                let result = match token {
                    Token::Plus => {
                        if rhs.is_percent {
                            lhs.num + lhs.num * rhs.num
                        } else {
                            lhs.num + rhs.num
                        }
                    }
                    Token::Minus => {
                        if rhs.is_percent {
                            lhs.num - lhs.num * rhs.num
                        } else {
                            lhs.num - rhs.num
                        }
                    }
                    Token::Star => lhs.num * rhs.num,
                    Token::Slash => lhs.num / rhs.num,
                    Token::Modulo => lhs.num % rhs.num,
                    Token::Power => lhs.num.powf(rhs.num),
                    // Unreachable: constrained by the outer match arm.
                    _ => return Err(fail(format!("Unknown token: {:?}", token))),
                };
                stack.push(plain(result));
            }
            // Structural tokens have no business in an RPN stream.
            Token::Y
            | Token::Equal
            | Token::Comma
            | Token::OpenParen
            | Token::CloseParen
            | Token::Pipe
            | Token::End => {
                return Err(fail(format!("Unexpected token in evaluation: {:?}", token)));
            }
        }
    }

    if token_count == 0 {
        return Err(EquationError::new("Invalid equation supplied"));
    }

    if stack.len() != 1 {
        return Err(EquationError::new(format!(
            "Invalid evaluation: expected 1 result, found {} items in stack",
            stack.len()
        )));
    }

    stack
        .pop()
        .map(|v| v.num)
        .ok_or_else(|| EquationError::new("Evaluation stack is empty"))
}
