#[derive(Debug, PartialEq)]
pub(crate) struct Operand {
    pub(crate) token: String,
    pub(crate) prec: usize,
    pub(crate) assoc: String,
    pub(crate) is_func: bool,
}

pub(crate) fn get_operator(operator: &str) -> Operand {
    match operator {
        "+" => add_op(),
        "-" => sub_op(),
        "*" => mul_op(),
        "/" => div_op(),
        "^" => pow_op(),
        "(" => left_par_op(),
        "sin" => sin_op(),
        "cos" => cos_op(),
        "tan" => tan_op(),
        "max" => max_op(),
        "abs" => abs_op(),
        "sqrt" => sqrt_op(),
        e => panic!("unknown operator {}", e)
    }
}

fn sqrt_op() -> Operand {
    Operand {
        token: String::from("sqrt"),
        prec: 0,
        assoc: String::from("r"),
        is_func: true,
    }
}

fn abs_op() -> Operand {
    Operand {
        token: String::from("abs"),
        prec: 0,
        assoc: String::from("r"),
        is_func: true,
    }
}

fn max_op() -> Operand {
    Operand {
        token: String::from("max"),
        prec: 0,
        assoc: String::from("r"),
        is_func: true,
    }
}

fn sin_op() -> Operand {
    Operand {
        token: String::from("sin"),
        prec: 0,
        assoc: String::from("r"),
        is_func: true,

    }
}

fn cos_op() -> Operand {
    Operand {
        token: String::from("cos"),
        prec: 0,
        assoc: String::from("r"),
        is_func: true,

    }
}

fn tan_op() -> Operand {
    Operand {
        token: String::from("tan"),
        prec: 0,
        assoc: String::from("r"),
        is_func: true,

    }
}

fn pow_op() -> Operand {
    Operand {
        token: String::from("^"),
        prec: 4,
        assoc: String::from("r"),
        is_func: false,

    }
}

fn div_op() -> Operand {
    Operand {
        token: String::from("/"),
        prec: 3,
        assoc: String::from("l"),
        is_func: false,

    }
}

fn mul_op() -> Operand {
    Operand {
        token: String::from("*"),
        prec: 3,
        assoc: String::from("l"),
        is_func: false,

    }
}

fn add_op() -> Operand {
    Operand {
        token: String::from("+"),
        prec: 2,
        assoc: String::from("l"),
        is_func: false,

    }
}

fn sub_op() -> Operand {
    Operand {
        token: String::from("-"),
        prec: 2,
        assoc: String::from("l"),
        is_func: false,

    }
}

fn left_par_op() -> Operand {
    Operand {
        token: String::from("("),
        prec: 0,
        assoc: String::from("l"),
        is_func: false,

    }
}