use std::{fmt::Debug, process::exit};

use crate::frontend::types::{Expression, LiteralTypes, Statement, TokenTypes, VariableTypes};

#[derive(Debug)]
pub struct Transpiler {
    ast: Statement,
    pub c_src_code: String,
    variables: Vec<Variable>,
    var_names: Vec<String>,
    functions: Vec<Variable>,
    func_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Expr {
    value: String,
    literal_type: VariableTypes,
}

#[derive(Debug, Clone, PartialEq)]
struct Variable {
    name: String,
    var_type: VariableTypes,
}

impl Transpiler {
    pub fn new(tree: Statement) -> Self {
        Self {
            ast: tree,
            c_src_code: String::from("#include \"stdlib.h\"\n"),
            variables: Vec::new(),
            var_names: Vec::new(),
            functions: Vec::new(),
            func_names: Vec::new(),
        }
    }

    pub fn transpile_abstract_syntax_tree(&mut self) {
        let stmts = self.get_body().to_vec();

        let c_code: String = stmts
            .iter()
            .map(|st| {
                //
                self.transpile_stmt(st)
            })
            .collect();

        self.c_src_code.push_str(&c_code);
    }

    fn transpile_stmt(&mut self, stmt: &Statement) -> String {
        let mut c_stmt = String::new();

        match stmt {
            Statement::FunctionDeclaration {
                name,
                r#type,
                params,
                body,
                ..
            } => {
                let c_type = self.get_c_type(r#type);
                // let c_params = params.iter().for_each(|p| )

                let bd: String = if let Some(b) = body {
                    (*b).iter().map(|st| self.transpile_stmt(st)).collect()
                } else {
                    String::new()
                };
                let c_body = bd;

                c_stmt.push_str(&format!("{} {}(){{{}}}", c_type, name, c_body).to_string());
            }
            Statement::VariableDeclaration {
                name,
                kind,
                r#type,
                value,
                ..
            } => {
                ////// TODO:
                // set variable on variables array on both BOTH!
            }
            Statement::FunctionCall(fc) => match fc {
                Expression::Call { name, arguments } => {
                    let mut c_args = match arguments {
                        Some(a) => (*a)
                            .iter()
                            .map(|e| format!("{},", self.eval_expr(e).value))
                            .collect(),
                        None => String::new(),
                    };

                    c_args.pop();

                    c_stmt.push_str(format!("{}({});", name, c_args).as_str())
                }
                _ => {}
            },
            _ => {}
        };

        c_stmt
    }

    fn eval_expr(&self, expr: &Expression) -> Expr {
        match expr {
            Expression::Literal { r#type, value } => match r#type {
                LiteralTypes::String => Expr {
                    value: format!("\"{}\"", value),
                    literal_type: VariableTypes::Str,
                },
                LiteralTypes::Null => Expr {
                    value: value.to_string(),
                    literal_type: VariableTypes::Nul,
                },
                LiteralTypes::Numeric => {
                    if value.contains('.') {
                        Expr {
                            value: value.to_string(),
                            literal_type: VariableTypes::Flo,
                        }
                    } else {
                        Expr {
                            value: value.to_string(),
                            literal_type: VariableTypes::Int,
                        }
                    }
                }
                LiteralTypes::Boolean => Expr {
                    value: value.to_string(),
                    literal_type: VariableTypes::Boo,
                },
            },
            Expression::ArrayLiteral { elements } => {
                unimplemented!()
                // match elements {
                //     Some(e) => {
                //         &*e
                //     }
                //     None => {
                //         //
                //     }
                // }
            }
            Expression::Identifier(name) => {
                if !&self.var_names.contains(name) {
                    self.error_expr(
                        format!("Variable '{}' being used before assigned", name).as_str(),
                    );
                    exit(1)
                } else {
                    return Expr {
                        value: name.to_string(),
                        literal_type: self.get_var_type(&name),
                    };
                }
            }
            Expression::Unary { operator, operand } => match operator {
                TokenTypes::LogicalNot => {
                    let right = self.eval_expr(operand);
                    if right.literal_type.eq(&VariableTypes::Boo) {
                        return Expr {
                            value: format!("!{}", right.value),
                            literal_type: right.literal_type,
                        };
                    } else {
                        self.error_expr("Cannot use '!' on non boolean values");
                        exit(1)
                    }
                }
                TokenTypes::BinaryMinus => {
                    let right = self.eval_expr(operand);
                    if right.literal_type.eq(&VariableTypes::Int)
                        || right.literal_type.eq(&VariableTypes::Flo)
                    {
                        return Expr {
                            value: format!("-{}", right.value),
                            literal_type: right.literal_type,
                        };
                    } else {
                        self.error_expr("Cannot use '-' on non-numeric value");
                        exit(1)
                    }
                }
                _ => {
                    eprintln!("Unknown error evaluating unary expression");
                    exit(1)
                }
            },
            Expression::Logical {
                operator,
                left,
                right,
            } => {
                let e_left = self.eval_expr(left);
                let e_right = self.eval_expr(right);

                match operator {
                    TokenTypes::LogicalEquals => match e_left.literal_type {
                        VariableTypes::Str => match e_right.literal_type {
                            VariableTypes::Str => {
                                return Expr {
                                    value: format!(
                                        "(compare({}, {}) == 0))",
                                        e_left.value, e_right.value
                                    ),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                            _ => {
                                return Expr {
                                    value: String::from("false"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                        },
                        VariableTypes::Int | VariableTypes::Flo => match e_right.literal_type {
                            VariableTypes::Int | VariableTypes::Flo => {
                                return Expr {
                                    value: format!("({} == {})", e_left.value, e_right.value),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                            _ => {
                                return Expr {
                                    value: String::from("false"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                        },
                        VariableTypes::Nul => match e_right.literal_type {
                            VariableTypes::Nul => {
                                return Expr {
                                    value: String::from("true"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                            _ => {
                                return Expr {
                                    value: String::from("false"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                        },
                        VariableTypes::Boo => match e_right.literal_type {
                            VariableTypes::Boo => {
                                return Expr {
                                    value: format!("({} == {})", e_left.value, e_right.value),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                            _ => {
                                return Expr {
                                    value: String::from("false"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                        },
                        _ => {
                            self.error_expr(
                                format!(
                                    "Cannot compare '{}' with '{}'",
                                    e_left.value, e_right.value,
                                )
                                .as_str(),
                            );
                            exit(1)
                        }
                    },
                    TokenTypes::LogicalDifferent => match e_left.literal_type {
                        VariableTypes::Str => match e_right.literal_type {
                            VariableTypes::Str => {
                                return Expr {
                                    value: format!(
                                        "(compare({}, {}) != 0))",
                                        e_left.value, e_right.value
                                    ),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                            _ => {
                                return Expr {
                                    value: String::from("true"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                        },
                        VariableTypes::Int | VariableTypes::Flo => match e_right.literal_type {
                            VariableTypes::Int | VariableTypes::Flo => {
                                return Expr {
                                    value: format!("({} != {})", e_left.value, e_right.value),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                            _ => {
                                return Expr {
                                    value: String::from("true"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                        },
                        VariableTypes::Nul => match e_right.literal_type {
                            VariableTypes::Nul => {
                                return Expr {
                                    value: String::from("false"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                            _ => {
                                return Expr {
                                    value: String::from("true"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                        },
                        VariableTypes::Boo => match e_right.literal_type {
                            VariableTypes::Boo => {
                                return Expr {
                                    value: format!("({} != {})", e_left.value, e_right.value),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                            _ => {
                                return Expr {
                                    value: String::from("true"),
                                    literal_type: VariableTypes::Boo,
                                };
                            }
                        },
                        _ => {
                            self.error_expr(
                                format!(
                                    "Cannot compare '{}' with '{}'",
                                    e_left.value, e_right.value,
                                )
                                .as_str(),
                            );
                            exit(1)
                        }
                    },
                    TokenTypes::LogicalSmallerThan
                    | TokenTypes::LogicalSmallerOrEqualsThan
                    | TokenTypes::LogicalGreaterThan
                    | TokenTypes::LogicalGreaterOrEqualsThan => {
                        // let op: TokenTypes = match operator {
                        //     TokenTypes::LogicalSmallerThan => TokenTypes::LogicalSmallerThan,
                        //     TokenTypes::LogicalSmallerOrEqualsThan => {
                        //         TokenTypes::LogicalSmallerOrEqualsThan
                        //     }
                        //     TokenTypes::LogicalGreaterThan => TokenTypes::LogicalGreaterThan,
                        //     TokenTypes::LogicalGreaterOrEqualsThan => {
                        //         TokenTypes::LogicalGreaterOrEqualsThan
                        //     }
                        //     _ => {
                        //         eprintln!("Unknown error related to logical expressions");
                        //         exit(1)
                        //     }
                        // };

                        match e_left.literal_type {
                            VariableTypes::Str => match e_right.literal_type {
                                VariableTypes::Str => {
                                    eprintln!(
                                        "Cannot compare strings with '{}', did you mean '=='?",
                                        operator
                                    );
                                    exit(1)
                                }
                                VariableTypes::Int | VariableTypes::Flo => {
                                    eprintln!(
                                        "Cannot compare '{}' with '{}', did you mean 'getLen({}) {} {}'?",
                                        e_left.value,
                                        e_right.value,
                                        operator,
                                        e_left.value,
                                        e_right.value
                                    );
                                    exit(1)
                                }
                                _ => {
                                    eprintln!(
                                        "Cannot compare '{}' with '{}'",
                                        e_left.value, e_right.value
                                    );
                                    exit(1)
                                }
                            },
                            VariableTypes::Int => match e_right.literal_type {
                                VariableTypes::Int => Expr {
                                    value: format!(
                                        "({} {} {})",
                                        e_left.value, operator, e_right.value
                                    ),
                                    literal_type: VariableTypes::Int,
                                },
                                VariableTypes::Flo => Expr {
                                    value: format!(
                                        "({} {} {})",
                                        e_left.value, operator, e_right.value
                                    ),
                                    literal_type: VariableTypes::Flo,
                                },
                                VariableTypes::Str => {
                                    eprintln!(
                                        "Cannot compare '{}' with '{}', did you mean '{} {} getLen({})'?",
                                        e_left.value,
                                        e_right.value,
                                        operator,
                                        e_left.value,
                                        e_right.value
                                    );
                                    exit(1)
                                }
                                _ => {
                                    eprintln!(
                                        "Cannot compare '{}' with '{}'",
                                        e_left.value, e_right.value
                                    );
                                    exit(1)
                                }
                            },
                            _ => {
                                eprintln!(
                                    "Cannot compare '{}' with '{}'",
                                    e_left.value, e_right.value
                                );
                                exit(1)
                            }
                        }
                    }
                    _ => {
                        eprintln!("Unknown error related to logical expressions");
                        exit(1)
                    }
                }
            }
            Expression::Binary {
                operator,
                left,
                right,
            } => {
                let e_right = self.eval_expr(right);
                let e_left = self.eval_expr(left);

                match operator {
                    TokenTypes::BinaryPlus => match e_left.literal_type {
                        VariableTypes::Str => match e_right.literal_type {
                            VariableTypes::Str => {
                                return Expr {
                                    value: format!("(concat({}, {}))", e_left.value, e_right.value),
                                    literal_type: VariableTypes::Str,
                                };
                            }
                            VariableTypes::Int | VariableTypes::Flo => {
                                return Expr {
                                    value: format!(
                                        "(concat({}, \"{}\"))",
                                        e_left.value, e_right.value
                                    ),
                                    literal_type: VariableTypes::Str,
                                };
                            }
                            _ => {
                                self.error_expr(
                                    format!(
                                        "Cannot concatenate '{}' with '{}'",
                                        e_left.value, e_right.value,
                                    )
                                    .as_str(),
                                );
                                exit(1)
                            }
                        },
                        VariableTypes::Int => match e_right.literal_type {
                            VariableTypes::Int => {
                                return Expr {
                                    value: format!(
                                        "({} {} {})",
                                        e_left.value, operator, e_right.value
                                    ),
                                    literal_type: VariableTypes::Int,
                                };
                            }
                            VariableTypes::Flo => {
                                return Expr {
                                    value: format!(
                                        "({} {} {})",
                                        e_left.value, operator, e_right.value
                                    ),
                                    literal_type: VariableTypes::Flo,
                                };
                            }
                            VariableTypes::Str => {
                                return Expr {
                                    value: format!(
                                        "(concat(\"{}\", {}))",
                                        e_left.value, e_right.value
                                    ),
                                    literal_type: VariableTypes::Str,
                                };
                            }
                            _ => {
                                self.error_expr(
                                    format!(
                                        "Cannot add '{}' with '{}'",
                                        e_left.value, e_right.value
                                    )
                                    .as_str(),
                                );
                                exit(1)
                            }
                        },
                        VariableTypes::Flo => match e_right.literal_type {
                            VariableTypes::Int | VariableTypes::Flo => {
                                return Expr {
                                    value: format!(
                                        "({} {} {})",
                                        e_left.value, operator, e_right.value
                                    ),
                                    literal_type: VariableTypes::Flo,
                                };
                            }
                            VariableTypes::Str => {
                                return Expr {
                                    value: format!(
                                        "(concat(\"{}\", {}))",
                                        e_left.value, e_right.value
                                    ),
                                    literal_type: VariableTypes::Str,
                                };
                            }
                            _ => {
                                self.error_expr(
                                    format!(
                                        "Cannot add '{}' with '{}'",
                                        e_left.value, e_right.value,
                                    )
                                    .as_str(),
                                );
                                exit(1)
                            }
                        },
                        _ => {
                            self.error_expr(
                                format!("Cannot add '{}' with '{}'", e_left.value, e_right.value,)
                                    .as_str(),
                            );
                            exit(1)
                        }
                    },
                    TokenTypes::BinaryMinus
                    | TokenTypes::BinaryDivision
                    | TokenTypes::BinaryMultiply
                    | TokenTypes::BinaryRest => match e_left.literal_type {
                        VariableTypes::Int => match e_right.literal_type {
                            VariableTypes::Int => {
                                return Expr {
                                    value: format!(
                                        "({} {} {})",
                                        e_left.value, operator, e_right.value
                                    ),
                                    literal_type: VariableTypes::Int,
                                };
                            }
                            VariableTypes::Flo => {
                                return Expr {
                                    value: format!(
                                        "({} {} {})",
                                        e_left.value, operator, e_right.value
                                    ),
                                    literal_type: VariableTypes::Flo,
                                };
                            }
                            _ => match operator {
                                TokenTypes::BinaryMinus => {
                                    self.error_expr(
                                        format!(
                                            "Cannot subtract '{}' with '{}'",
                                            e_left.value, e_right.value,
                                        )
                                        .as_str(),
                                    );
                                    exit(1)
                                }
                                TokenTypes::BinaryMultiply => {
                                    self.error_expr(
                                        format!(
                                            "Cannot multiply '{}' with '{}'",
                                            e_left.value, e_right.value,
                                        )
                                        .as_str(),
                                    );
                                    exit(1)
                                }
                                TokenTypes::BinaryDivision => {
                                    self.error_expr(
                                        format!(
                                            "Cannot divide '{}' with '{}'",
                                            e_left.value, e_right.value,
                                        )
                                        .as_str(),
                                    );
                                    exit(1)
                                }
                                TokenTypes::BinaryRest => {
                                    self.error_expr(
                                        format!(
                                            "Cannot take modulo of '{}' with '{}'",
                                            e_left.value, e_right.value,
                                        )
                                        .as_str(),
                                    );
                                    exit(1)
                                }
                                _ => {
                                    eprintln!(
                                        "Unknown error related with numeric binary operations"
                                    );
                                    exit(1)
                                }
                            },
                        },
                        VariableTypes::Flo => match e_right.literal_type {
                            VariableTypes::Int | VariableTypes::Flo => {
                                return Expr {
                                    value: format!(
                                        "({} {} {})",
                                        e_left.value, operator, e_right.value
                                    ),
                                    literal_type: VariableTypes::Flo,
                                };
                            }
                            _ => match operator {
                                TokenTypes::BinaryMinus => {
                                    self.error_expr(
                                        format!(
                                            "Cannot subtract '{}' with '{}'",
                                            e_left.value, e_right.value,
                                        )
                                        .as_str(),
                                    );
                                    exit(1)
                                }
                                TokenTypes::BinaryMultiply => {
                                    self.error_expr(
                                        format!(
                                            "Cannot multiply '{}' with '{}'",
                                            e_left.value, e_right.value,
                                        )
                                        .as_str(),
                                    );
                                    exit(1)
                                }
                                TokenTypes::BinaryDivision => {
                                    self.error_expr(
                                        format!(
                                            "Cannot divide '{}' with '{}'",
                                            e_left.value, e_right.value,
                                        )
                                        .as_str(),
                                    );
                                    exit(1)
                                }
                                TokenTypes::BinaryRest => {
                                    self.error_expr(
                                        format!(
                                            "Cannot take modulo of '{}' with '{}'",
                                            e_left.value, e_right.value,
                                        )
                                        .as_str(),
                                    );
                                    exit(1)
                                }
                                _ => {
                                    eprintln!(
                                        "Unknown error related with numeric binary operations"
                                    );
                                    exit(1)
                                }
                            },
                        },
                        _ => {
                            self.error_expr(
                                format!(
                                    "Cannot perform this operation '{} {} {}'",
                                    e_left.value, operator, e_right.value,
                                )
                                .as_str(),
                            );
                            exit(1)
                        }
                    },
                    _ => {
                        self.error_expr("Unknown error related to binary operations");
                        exit(1)
                    }
                }
            }
            _ => {
                self.error_expr("Unknown error related to expression evaluation");
                exit(1)
            }
        }
    }

    fn error_expr(&self, message: &str) {
        eprintln!("\n| Error: {}", message);
    }

    fn get_var_type(&self, name: &str) -> VariableTypes {
        for i in &self.variables {
            if &i.name == &name {
                return i.var_type.to_owned();
            }
        }
        eprintln!(
            "{}",
            format!("Variable '{}' being used before assigned", name)
        );
        exit(1)
    }

    fn get_c_type(&self, bline_type: &VariableTypes) -> String {
        match bline_type {
            VariableTypes::Int => String::from("int"),
            _ => String::from("void"),
        }
    }

    fn get_body(&self) -> &Vec<Statement> {
        match &self.ast {
            Statement::Program { body, .. } => {
                //
                &body
            }
            _ => exit(1),
        }
    }
}
// I'M SO SORRY I DID NOT DISPLAY COL AND LINE ON ERRORS AT THIS FILE, my code before this was
// GARBAGE and i cannot fix it now
