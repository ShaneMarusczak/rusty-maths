use crate::equation_analyzer::structs::token::{Token, TokenType};

pub(crate) fn starts_or_ends_with_y(eq: &[Token]) -> bool {
    let len = eq.len();
    if eq[0].literal == "y" && eq[1].literal == "=" {
        return true;
    }
    // not -1 and -2 because of End token from tokenizer
    if eq[len - 2].literal == "y" && eq[len - 3].literal == "=" {
        return true;
    }
    false
}

pub(crate) fn get_multiplier(i: usize, eq: &[Token]) -> f32 {
    let mut multiplier = 1_f32;
    if i != 0 && eq[i - 1].token_type == TokenType::Minus {
        multiplier = -1_f32;
    }
    multiplier
}
