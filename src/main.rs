pub mod ast;
pub mod env;
pub mod eval;

use std::fmt;

use crate::ast::Term;
use crate::eval::Evaluator;

#[derive(Debug)]
pub struct EvalError(ast::Error);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ast::Location {
            start,
            end,
            filename,
        } = &self.0.location;
        let message = &self.0.message;
        let full_text = &self.0.full_text;
        writeln!(f, "[Error ({}:{}:{})] {}", filename, start, end, message)?;
        writeln!(f, "{}", full_text)?;
        Ok(())
    }
}

impl std::error::Error for EvalError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <json-file>", args[0]);
        return Ok(());
    }

    let contents = std::fs::read_to_string(args.pop().unwrap())?;
    let file: ast::File = serde_json::from_str(&contents)?;

    let mut env = Default::default();
    let term = file.expression;
    let result = Evaluator::eval(&mut env, term);

    match result {
        Term::Error(e) => Err(Box::new(EvalError(e))),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::*, eval::Evaluator};

    #[test]
    fn factorial() {
        let mut env = Default::default();
        let term = Term::Let(Let {
            name: Var {
                text: "factorial".into(),
                location: Location {
                    start: 0,
                    end: 0,
                    filename: Default::default(),
                },
            },
            value: Box::new(Term::Function(Function {
                parameters: vec![Var {
                    text: "n".into(),
                    location: Location {
                        start: 1,
                        end: 1,
                        filename: Default::default(),
                    },
                }],
                value: Box::new(Term::If(If {
                    condition: Box::new(Term::Binary(Binary {
                        lhs: Box::new(Term::Var(Var {
                            text: "n".into(),
                            location: Location {
                                start: 2,
                                end: 2,
                                filename: Default::default(),
                            },
                        })),
                        op: BinaryOp::Lte,
                        rhs: Box::new(Term::Int(Int {
                            value: 1,
                            location: Location {
                                start: 3,
                                end: 3,
                                filename: Default::default(),
                            },
                        })),
                        location: Location {
                            start: 4,
                            end: 4,
                            filename: Default::default(),
                        },
                    })),
                    then: Box::new(Term::Int(Int {
                        value: 1,
                        location: Location {
                            start: 5,
                            end: 5,
                            filename: Default::default(),
                        },
                    })),
                    otherwise: Box::new(Term::Binary(Binary {
                        lhs: Box::new(Term::Var(Var {
                            text: "n".into(),
                            location: Location {
                                start: 6,
                                end: 6,
                                filename: Default::default(),
                            },
                        })),
                        op: BinaryOp::Mul,
                        rhs: Box::new(Term::Call(Call {
                            callee: Box::new(Term::Var(Var {
                                text: "factorial".into(),
                                location: Location {
                                    start: 7,
                                    end: 7,
                                    filename: Default::default(),
                                },
                            })),
                            arguments: vec![Term::Binary(Binary {
                                lhs: Box::new(Term::Var(Var {
                                    text: "n".into(),
                                    location: Location {
                                        start: 8,
                                        end: 8,
                                        filename: Default::default(),
                                    },
                                })),
                                op: BinaryOp::Sub,
                                rhs: Box::new(Term::Int(Int {
                                    value: 1,
                                    location: Location {
                                        start: 9,
                                        end: 9,
                                        filename: Default::default(),
                                    },
                                })),
                                location: Location {
                                    start: 10,
                                    end: 10,
                                    filename: Default::default(),
                                },
                            })],
                            location: Location {
                                start: 11,
                                end: 11,
                                filename: Default::default(),
                            },
                        })),
                        location: Location {
                            start: 12,
                            end: 12,
                            filename: Default::default(),
                        },
                    })),
                    location: Location {
                        start: 13,
                        end: 13,
                        filename: Default::default(),
                    },
                })),
                location: Location {
                    start: 14,
                    end: 14,
                    filename: Default::default(),
                },
            })),
            next: Box::new(Term::Call(Call {
                callee: Box::new(Term::Var(Var {
                    text: "factorial".into(),
                    location: Location {
                        start: 15,
                        end: 15,
                        filename: Default::default(),
                    },
                })),
                arguments: vec![Term::Int(Int {
                    value: 4,
                    location: Location {
                        start: 16,
                        end: 16,
                        filename: Default::default(),
                    },
                })],
                location: Location {
                    start: 17,
                    end: 17,
                    filename: Default::default(),
                },
            })),
            location: Location {
                start: 18,
                end: 18,
                filename: Default::default(),
            },
        });

        let result = Evaluator::eval(&mut env, term);
        let term = Term::Int(Int {
            value: 24,
            location: Location {
                start: 12,
                end: 12,
                filename: Default::default(),
            },
        });

        assert_eq!(term, result);
    }
}
