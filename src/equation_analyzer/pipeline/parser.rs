use crate::equation_analyzer::errors::{EquationError, Span};
use crate::equation_analyzer::structs::operands::{get_operator, Assoc, Operand};
use crate::equation_analyzer::structs::token::{Callee, SpannedToken, Token};

/// Represents a parser frame for a parenthesized function call — unary and
/// variadic alike. Tracks the backing callee so we can emit the matching
/// EndCall, and the call token's span so errors can underline the whole call.
struct ParserFrame {
    callee: Callee,
    call_span: Span,
    operator_stack_position: usize,
}

/// Is this a Call token whose sole arg comes off the value stack (no frame)?
/// User-defined functions are always unary (their one parameter is `x`).
fn is_unary_call(token: Token) -> bool {
    match token {
        Token::Call(Callee::Catalog(s)) => s.kind.is_unary(),
        Token::Call(Callee::User(_)) => true,
        _ => false,
    }
}

/// Pop operators to the output until a parenthesis opener (or an empty stack)
/// is reached. The opener itself stays on the stack.
fn pop_until_paren_opener(operator_stack: &mut Vec<Operand>, output: &mut Vec<SpannedToken>) {
    while let Some(top) = operator_stack.last() {
        if top.paren_opener {
            break;
        }
        if let Some(op) = operator_stack.pop() {
            output.push(op.token);
        }
    }
}

/// A comma is only valid directly inside a call's parentheses, separating
/// arguments. Anywhere else — grouping parens, `log_N(...)`, or bare — it's
/// an immediate error rather than a token to skip: digit grouping is spelled
/// `1_000`, and a silently dropped comma either mis-splits an enclosing
/// call's arguments or surfaces later as a confusing stack-shape error.
fn comma_error(operator_stack: &[Operand], span: Span) -> EquationError {
    let opener = operator_stack.iter().rev().find(|op| op.paren_opener);
    let message = match opener.map(|op| op.token.token) {
        Some(Token::Log { .. }) => String::from("log takes exactly one argument"),
        _ => String::from("Unexpected ','"),
    };
    EquationError::spanned(message, span)
}

/// Generic Shunting Yard parser that works with any iterator of tokens.
///
/// This is the core parsing logic shared by all pipeline implementations.
/// Converts infix notation to Reverse Polish Notation (RPN) using Dijkstra's Shunting Yard algorithm.
///
/// # Arguments
/// * `tokens` - An iterator yielding spanned tokens in infix notation
///
/// # Returns
/// * `Ok(Vec<SpannedToken>)` - The tokens in RPN format
/// * `Err(EquationError)` - An error (with the offending span where one exists)
///
/// # Algorithm
/// Uses Dijkstra's Shunting Yard algorithm to convert infix notation to RPN:
/// 1. Maintains an operator stack for pending operators
/// 2. Builds output queue of tokens in RPN order
/// 3. Handles operator precedence and associativity
/// 4. Manages parentheses and function calls
/// 5. Special handling for variadic functions (avg, min, max, etc.)
///
/// # Note
/// This algorithm requires buffering the output due to the nature of infix-to-RPN conversion.
/// The output is always a Vec, but the input can be any iterator (streaming or not).
pub(crate) fn parse<I>(tokens: I) -> Result<Vec<SpannedToken>, EquationError>
where
    I: IntoIterator<Item = Result<SpannedToken, EquationError>>,
{
    let mut operator_stack: Vec<Operand> = Vec::new();
    let mut output: Vec<SpannedToken> = Vec::new();
    let mut paren_depth = 0;
    let mut frames: Vec<ParserFrame> = Vec::new();
    let mut found_end = false;
    // Span of the pipe operator awaiting its function, when one is armed.
    let mut expect_piped_function: Option<Span> = None;

    for token_result in tokens {
        let spanned = token_result?;
        let token = spanned.token;

        // After a Pipe, the next token must be a unary Call. Emit it directly
        // to the output queue (it's already in postfix position).
        if expect_piped_function.is_some() {
            if is_unary_call(token) {
                output.push(spanned);
                expect_piped_function = None;
                continue;
            }
            return Err(EquationError::spanned(
                format!("Expected a unary function after '|>', got {:?}", token),
                spanned.span,
            ));
        }

        // Handle variadic function parameter collection
        // With frame-based evaluation, we now allow full expressions in parameters
        if let Some(frame) = frames.last() {
            match token {
                // Comma: pop all pending operators (they belong to current parameter expression)
                Token::Comma => {
                    pop_until_paren_opener(&mut operator_stack, &mut output);

                    // The comma must sit directly at this frame's boundary;
                    // one inside a nested unary call or plain parens would
                    // otherwise silently re-split the frame's arguments
                    // (`avg(1, sin(2,3))` must not become avg(1, 2, sin(3))).
                    if operator_stack.len() != frame.operator_stack_position + 1 {
                        return Err(comma_error(&operator_stack, spanned.span));
                    }
                    continue;
                }

                // CloseParen might end the variadic function, or a nested regular function
                Token::CloseParen => {
                    pop_until_paren_opener(&mut operator_stack, &mut output);

                    // Check if we've drained back to the frame boundary
                    if operator_stack.len() == frame.operator_stack_position + 1 {
                        operator_stack.pop();
                        paren_depth -= 1;
                        // The EndCall's span covers the whole call, from the
                        // function name through this closing paren.
                        output.push(SpannedToken::new(
                            Token::EndCall(frame.callee),
                            Span::new(frame.call_span.start, spanned.span.end),
                        ));
                        frames.pop();
                        continue;
                    }

                    // Otherwise, it's a regular function or parenthesis - fall through to normal processing
                }

                // All other tokens fall through to normal processing
                _ => {}
            }
        }

        match token {
            // Skip equation markers
            Token::Y | Token::Equal => continue,

            // A comma outside a call's parentheses is an error, not a
            // separator to skip (see comma_error).
            Token::Comma => return Err(comma_error(&operator_stack, spanned.span)),

            // Constants and operands go directly to output
            Token::Constant(_) | Token::Number(_) | Token::X => output.push(spanned),

            // Every parenthesized call — unary or variadic, catalog or
            // user-defined — starts a frame; the callee's arity is enforced
            // at the matching EndCall. The tokenizer already consumed the
            // OpenParen; we push a synthetic marker to fence off preceding
            // operators until the matching CloseParen.
            Token::Call(callee) => {
                output.push(SpannedToken::new(Token::CallStart(callee), spanned.span));
                frames.push(ParserFrame {
                    callee,
                    call_span: spanned.span,
                    operator_stack_position: operator_stack.len(),
                });
                operator_stack.push(get_operator(SpannedToken::new(
                    Token::OpenParen,
                    spanned.span,
                ))?);
                paren_depth += 1;
            }

            // log_N and opening parenthesis go on operator stack.
            Token::Log { .. } | Token::OpenParen => {
                paren_depth += 1;
                operator_stack.push(get_operator(spanned)?);
            }

            // Closing parenthesis: pop operators until matching open paren
            Token::CloseParen => {
                paren_depth -= 1;

                if paren_depth < 0 {
                    return Err(EquationError::spanned(
                        "Invalid closing parenthesis",
                        spanned.span,
                    ));
                }

                pop_until_paren_opener(&mut operator_stack, &mut output);

                // The fence is either a plain parenthesis (dropped) or a
                // pending function call / log_N (emitted now that its
                // argument is complete).
                let opener = operator_stack.pop().ok_or_else(|| {
                    EquationError::spanned("Mismatched parentheses", spanned.span)
                })?;
                if opener.is_func {
                    output.push(opener.token);
                }
            }

            // Operators: apply precedence and associativity rules
            Token::Star
            | Token::Slash
            | Token::Plus
            | Token::Minus
            | Token::UnaryMinus
            | Token::Power
            | Token::Modulo
            | Token::Percent
            | Token::Factorial => {
                let o_1 = get_operator(spanned)?;

                // Pop higher precedence operators from stack
                while let Some(last) = operator_stack.last() {
                    if last.paren_opener {
                        break;
                    }

                    // Precedence and associativity rules
                    let should_pop =
                        last.prec > o_1.prec || (last.prec == o_1.prec && o_1.assoc == Assoc::Left);

                    if !should_pop {
                        break;
                    }

                    let o_2_new = operator_stack.pop().ok_or_else(|| {
                        EquationError::new("Internal error: operator stack became empty")
                    })?;
                    output.push(o_2_new.token);
                }

                operator_stack.push(o_1);
            }

            // Pipe operator: flush pending operators (down to current paren depth)
            // and arm `expect_piped_function` so the next token is emitted directly.
            Token::Pipe => {
                pop_until_paren_opener(&mut operator_stack, &mut output);
                expect_piped_function = Some(spanned.span);
            }

            // End token marks completion
            Token::End => {
                found_end = true;
            }

            // Parser-synthesized tokens must never appear in the input stream
            Token::CallStart(_) | Token::EndCall(_) => {
                return Err(EquationError::spanned(
                    format!("Unexpected token in input: {:?}", token),
                    spanned.span,
                ));
            }
        }
    }

    // An unclosed call (`sqrt(4`) — its fence marker is still on the stack;
    // report it against the call itself.
    if let Some(frame) = frames.last() {
        return Err(EquationError::spanned("Invalid function", frame.call_span));
    }

    // Pop remaining operators from stack. A leftover parenthesis opener —
    // a bare `(` or an unclosed `log_N(` — is an error that points at the
    // opener itself.
    while let Some(op) = operator_stack.pop() {
        if op.paren_opener {
            let message = if matches!(op.token.token, Token::OpenParen) {
                "Invalid opening parenthesis"
            } else {
                "Invalid function"
            };
            return Err(EquationError::spanned(message, op.token.span));
        }
        output.push(op.token);
    }

    // Validation
    if paren_depth != 0 {
        return Err(EquationError::new("Invalid function"));
    }

    if !found_end {
        return Err(EquationError::new("No end token found"));
    }

    if let Some(pipe_span) = expect_piped_function {
        return Err(EquationError::spanned(
            "Dangling '|>': expected a unary function on the right side",
            pipe_span,
        ));
    }

    Ok(output)
}
