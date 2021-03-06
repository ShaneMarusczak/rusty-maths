pub(crate) struct Operand {
    pub(crate) token: String,
    pub(crate) prec: usize,
    pub(crate) assoc: String,
    pub(crate) is_func: bool,
    pub(crate) paren_opener: bool,
}

pub(crate) fn get_operator(operator: &str) -> Operand {
    match operator {
        "(" => get_op(operator, 0, "r", false, true),
        "+" | "-" => get_op(operator, 2, "l", false, false),
        "*" | "/" | "%" | "%%" => get_op(operator, 3, "l", false, false),
        "^" => get_op(operator, 4, "r", false, false),
        "sqrt(" | "ln(" | "abs(" | "max(" | "min(" | "sin(" | "cos(" | "tan(" => {
            get_op(operator, 0, "r", true, true)
        }
        op => {
            if op.starts_with("log_") {
                get_op(operator, 0, "r", true, true)
            } else {
                panic!("unknown operator {}", op);
            }
        }
    }
}

fn get_op(token: &str, prec: usize, assoc: &str, is_func: bool, paren_opener: bool) -> Operand {
    Operand {
        token: token.to_string(),
        prec,
        assoc: assoc.to_string(),
        is_func,
        paren_opener,
    }
}
