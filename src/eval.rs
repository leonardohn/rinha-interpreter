use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::*;
use crate::env::Env;

#[derive(Debug)]
pub struct Evaluator;

impl Evaluator {
    pub fn eval(env: &mut Rc<RefCell<Env>>, term: Term) -> Term {
        match term {
            term @ Term::Int(_) => term,
            term @ Term::Str(_) => term,
            term @ Term::Bool(_) => term,
            term @ Term::Error(_) => term,
            term @ Term::Function(_) => term,
            Term::If(term) => Self::eval_if(env, term),
            Term::Let(term) => Self::eval_let(env, term),
            Term::Var(term) => Self::eval_var(env, term),
            Term::Call(term) => Self::eval_call(env, term),
            Term::First(term) => Self::eval_first(env, term),
            Term::Print(term) => Self::eval_print(env, term),
            Term::Tuple(term) => Self::eval_tuple(env, term),
            Term::Binary(term) => Self::eval_binary(env, term),
            Term::Second(term) => Self::eval_second(env, term),
        }
    }

    fn eval_if(env: &mut Rc<RefCell<Env>>, term: If) -> Term {
        match Self::eval(env, *term.condition) {
            Term::Bool(Bool { value: true, .. }) => Self::eval(env, *term.then),
            Term::Bool(Bool { value: false, .. }) => {
                Self::eval(env, *term.otherwise)
            }
            term => {
                let message = "Unexpected term".into();
                let full_text = "Expected condition of type \"Bool\"".into();
                error(term, message, full_text)
            }
        }
    }

    fn eval_call(env: &mut Rc<RefCell<Env>>, term: Call) -> Term {
        match Self::eval(env, *term.callee) {
            Term::Function(Function {
                parameters,
                value,
                location,
            }) => {
                let expected_args = parameters.len();
                let found_args = term.arguments.len();

                if expected_args != found_args {
                    let message = "Argument count mismatch".into();
                    let full_text = format!(
                        "Expected {expected_args} arguments, found {found_args}"
                    );
                    let term = Term::Call(Call {
                        callee: Box::new(Term::Function(Function {
                            parameters,
                            value,
                            location,
                        })),
                        arguments: term.arguments,
                        location: term.location,
                    });
                    return error(term, message, full_text);
                }

                let mut env = Rc::new(RefCell::new(Env::extend(env.clone())));
                let pairs = term.arguments.into_iter().zip(parameters.iter());

                for (arg, param) in pairs {
                    let name = &param.text;
                    let value = Self::eval(&mut env, arg);
                    env.borrow_mut().set(name, value);
                }

                Self::eval(&mut env, *value)
            }
            term => {
                let message = "Unexpected term".into();
                let full_text = "Expected function body or reference".into();
                error(term, message, full_text)
            }
        }
    }

    fn eval_binary(env: &mut Rc<RefCell<Env>>, term: Binary) -> Term {
        match term.op {
            BinaryOp::Add => Self::eval_add(env, term),
            BinaryOp::Sub => Self::eval_sub(env, term),
            BinaryOp::Mul => Self::eval_mul(env, term),
            BinaryOp::Div => Self::eval_div(env, term),
            BinaryOp::Rem => Self::eval_rem(env, term),
            BinaryOp::Eq => Self::eval_eq(env, term),
            BinaryOp::Neq => Self::eval_neq(env, term),
            BinaryOp::Lt => Self::eval_lt(env, term),
            BinaryOp::Gt => Self::eval_gt(env, term),
            BinaryOp::Lte => Self::eval_lte(env, term),
            BinaryOp::Gte => Self::eval_gte(env, term),
            BinaryOp::And => Self::eval_and(env, term),
            BinaryOp::Or => Self::eval_or(env, term),
        }
    }

    fn eval_eq(env: &mut Rc<RefCell<Env>>, term: Binary) -> Term {
        let value = Self::eval(env, *term.lhs) == Self::eval(env, *term.rhs);
        let location = term.location;
        Term::Bool(Bool { value, location })
    }

    fn eval_neq(env: &mut Rc<RefCell<Env>>, term: Binary) -> Term {
        let value = Self::eval(env, *term.lhs) != Self::eval(env, *term.rhs);
        let location = term.location;
        Term::Bool(Bool { value, location })
    }

    fn eval_let(env: &mut Rc<RefCell<Env>>, term: Let) -> Term {
        let value = Self::eval(env, *term.value);
        let name = &term.name.text;

        if name != "_" {
            env.borrow_mut().set(name, value);
        }

        Self::eval(env, *term.next)
    }

    fn eval_print(env: &mut Rc<RefCell<Env>>, term: Print) -> Term {
        let term = Self::eval(env, *term.value);

        match &term {
            &Term::Error(_) => term,
            &Term::Int(Int { ref value, .. }) => {
                println!("{value}");
                term
            }
            &Term::Str(Str { ref value, .. }) => {
                println!("{value}");
                term
            }
            &Term::Bool(Bool { ref value, .. }) => {
                println!("{value}");
                term
            }
            &Term::Function(Function { .. }) => {
                println!("<function>");
                term
            }
            _ => {
                let message = "Unexpected term".into();
                let full_text = "The term is not a first class value".into();
                error(term, message, full_text)
            }
        }
    }

    fn eval_first(env: &mut Rc<RefCell<Env>>, term: First) -> Term {
        match Self::eval(env, *term.value) {
            Term::Tuple(v) => Self::eval(env, *v.first),
            term => {
                let message = "Unexpected term".into();
                let full_text = "The first function expects a tuple".into();
                error(term, message, full_text)
            }
        }
    }

    fn eval_second(env: &mut Rc<RefCell<Env>>, term: Second) -> Term {
        match Self::eval(env, *term.value) {
            Term::Tuple(v) => Self::eval(env, *v.second),
            term => {
                let message = "Unexpected term".into();
                let full_text = "The second function expects a tuple".into();
                error(term, message, full_text)
            }
        }
    }

    fn eval_tuple(env: &mut Rc<RefCell<Env>>, term: Tuple) -> Term {
        Term::Tuple(Tuple {
            first: Box::new(Self::eval(env, *term.first)),
            second: Box::new(Self::eval(env, *term.second)),
            location: term.location,
        })
    }

    fn eval_var(env: &mut Rc<RefCell<Env>>, term: Var) -> Term {
        let Var { text, location } = term;
        let value = env.borrow().get(&text);
        match value {
            Some(term) => Self::eval(env, term),
            None => {
                let message = "Undefined variable".into();
                let full_text = format!("Undefined variable \"{text}\"");
                error(Term::Var(Var { text, location }), message, full_text)
            }
        }
    }
}

macro_rules! impl_binary_op {
    ($($id:ident [($lhs:ident, $rhs:ident) => $out:ident] = $ev:expr;)*) => {
        impl Evaluator {$(
            fn $id(env: &mut Rc<RefCell<Env>>, term: Binary) -> Term {
                let lhs = match Self::eval(env, *term.lhs) {
                    term @ Term::Error(_) => return term,
                    Term::$lhs($lhs { value, .. }) => value,
                    term => {
                        let message = "Unexpected left operand".into();
                        let full_text = format!(
                            "Expected operand of type \"{}\"",
                            stringify!($lhs),
                        );
                        return error(term, message, full_text);
                    }
                };
                let rhs = match Self::eval(env, *term.rhs) {
                    term @ Term::Error(_) => return term,
                    Term::$rhs($rhs { value, .. }) => value,
                    term => {
                        let message = "Unexpected right operand".into();
                        let full_text = format!(
                            "Expected operand of type \"{}\"",
                            stringify!($rhs),
                        );
                        return error(term, message, full_text);
                    }
                };
                let value = $ev(lhs, rhs);
                let location = term.location;
                Term::$out($out { value, location })
            }
        )*}
    };
}

impl_binary_op! {
    eval_add[(Int, Int) => Int] = i32::wrapping_add;
    eval_sub[(Int, Int) => Int] = i32::wrapping_sub;
    eval_mul[(Int, Int) => Int] = i32::wrapping_mul;
    eval_div[(Int, Int) => Int] = i32::wrapping_div;
    eval_rem[(Int, Int) => Int] = i32::wrapping_rem;
    eval_lt[(Int, Int) => Bool] = |lhs, rhs| lhs < rhs;
    eval_gt[(Int, Int) => Bool] = |lhs, rhs| lhs > rhs;
    eval_lte[(Int, Int) => Bool] = |lhs, rhs| lhs <= rhs;
    eval_gte[(Int, Int) => Bool] = |lhs, rhs| lhs >= rhs;
    eval_or[(Bool, Bool) => Bool] = |lhs, rhs| lhs || rhs;
    eval_and[(Bool, Bool) => Bool] = |lhs, rhs| lhs && rhs;
}

pub fn error(term: Term, message: String, full_text: String) -> Term {
    match term {
        term @ Term::Error(_) => term,
        Term::If(If { location, .. })
        | Term::Let(Let { location, .. })
        | Term::Int(Int { location, .. })
        | Term::Str(Str { location, .. })
        | Term::Var(Var { location, .. })
        | Term::Bool(Bool { location, .. })
        | Term::Call(Call { location, .. })
        | Term::First(First { location, .. })
        | Term::Print(Print { location, .. })
        | Term::Tuple(Tuple { location, .. })
        | Term::Binary(Binary { location, .. })
        | Term::Second(Second { location, .. })
        | Term::Function(Function { location, .. }) => Term::Error(Error {
            message,
            full_text,
            location,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_int() {
        let mut env = Default::default();
        let value = Default::default();
        let location = Default::default();
        let term = Term::Int(Int { value, location });
        let result = Evaluator::eval(&mut env, term.clone());
        assert_eq!(term, result);
    }

    #[test]
    fn eval_str() {
        let mut env = Default::default();
        let value = Default::default();
        let location = Default::default();
        let term = Term::Str(Str { value, location });
        let result = Evaluator::eval(&mut env, term.clone());
        assert_eq!(term, result);
    }

    #[test]
    fn eval_bool() {
        let mut env = Default::default();
        let value = Default::default();
        let location = Default::default();
        let term = Term::Bool(Bool { value, location });
        let result = Evaluator::eval(&mut env, term.clone());
        assert_eq!(term, result);
    }

    #[test]
    fn eval_error() {
        let mut env = Default::default();
        let message = Default::default();
        let full_text = Default::default();
        let location = Default::default();
        let term = Term::Error(Error {
            message,
            full_text,
            location,
        });
        let result = Evaluator::eval(&mut env, term.clone());
        assert_eq!(term, result);
    }

    #[test]
    fn eval_tuple() {
        let mut env = Default::default();
        let term = Term::Tuple(Tuple {
            first: Box::new(Term::Int(Int {
                value: 1,
                location: Default::default(),
            })),
            second: Box::new(Term::Int(Int {
                value: 2,
                location: Default::default(),
            })),
            location: Default::default(),
        });
        let result = Evaluator::eval(&mut env, term.clone());
        assert_eq!(term, result);
    }

    #[test]
    fn eval_first() {
        let mut env = Default::default();
        let term = Term::First(First {
            value: Box::new(Term::Tuple(Tuple {
                first: Box::new(Term::Int(Int {
                    value: 1,
                    location: Default::default(),
                })),
                second: Box::new(Term::Int(Int {
                    value: 2,
                    location: Default::default(),
                })),
                location: Default::default(),
            })),
            location: Default::default(),
        });
        let result = Evaluator::eval(&mut env, term.clone());
        let term = Term::Int(Int {
            value: 1,
            location: Default::default(),
        });
        assert_eq!(term, result);
    }

    #[test]
    fn eval_second() {
        let mut env = Default::default();
        let term = Term::Second(Second {
            value: Box::new(Term::Tuple(Tuple {
                first: Box::new(Term::Int(Int {
                    value: 1,
                    location: Default::default(),
                })),
                second: Box::new(Term::Int(Int {
                    value: 2,
                    location: Default::default(),
                })),
                location: Default::default(),
            })),
            location: Default::default(),
        });
        let result = Evaluator::eval(&mut env, term.clone());
        let term = Term::Int(Int {
            value: 2,
            location: Default::default(),
        });
        assert_eq!(term, result);
    }

    #[test]
    fn eval_let_var() {
        let mut env = Default::default();
        let term = Term::Let(Let {
            name: Var {
                text: "foo".into(),
                location: Default::default(),
            },
            value: Box::new(Term::Int(Int {
                value: 42,
                location: Default::default(),
            })),
            next: Box::new(Term::Var(Var {
                text: "foo".into(),
                location: Default::default(),
            })),
            location: Default::default(),
        });
        let result = Evaluator::eval(&mut env, term);
        let term = Term::Int(Int {
            value: 42,
            location: Default::default(),
        });
        assert_eq!(term, result);
    }

    #[test]
    fn eval_let_call() {
        let mut env = Default::default();
        let term = Term::Let(Let {
            name: Var {
                text: "id".into(),
                location: Default::default(),
            },
            value: Box::new(Term::Function(Function {
                parameters: vec![Var {
                    text: "x".into(),
                    location: Default::default(),
                }],
                value: Box::new(Term::Var(Var {
                    text: "x".into(),
                    location: Default::default(),
                })),
                location: Default::default(),
            })),
            next: Box::new(Term::Call(Call {
                callee: Box::new(Term::Var(Var {
                    text: "id".into(),
                    location: Default::default(),
                })),
                arguments: vec![Term::Int(Int {
                    value: 42,
                    location: Default::default(),
                })],
                location: Default::default(),
            })),
            location: Default::default(),
        });
        let result = Evaluator::eval(&mut env, term);
        let term = Term::Int(Int {
            value: 42,
            location: Default::default(),
        });
        assert_eq!(term, result);
    }

    macro_rules! impl_eval_binary {
        ($($id:ident [$opd:ident; ($lhs:ident, $rhs:ident) => $out:ident] = {
            $(($op1:expr, $op2:expr$(,)?) => $res:expr;)*
        };)*) => {$(
            #[test]
            fn $id() {$(
                let mut env = Default::default();
                let op = BinaryOp::$opd;
                let value = $op1;
                let location = Default::default();
                let lhs = Box::new(Term::$lhs($lhs { value, location }));
                let value = $op2;
                let location = Default::default();
                let rhs = Box::new(Term::$rhs($rhs { value, location }));
                let location = Default::default();
                let term = Term::Binary(Binary {
                    lhs,
                    op,
                    rhs,
                    location,
                });
                let result = Evaluator::eval(&mut env, term);
                let value = $res;
                let location = Default::default();
                let term = Term::$out($out { value, location });
                assert_eq!(term, result);
            )*})*
        };
    }

    impl_eval_binary! {
        eval_add[Add; (Int, Int) => Int] = {
            (1, 1) => 2;
            (1, -1) => 0;
            (-1, -1) => -2;
            (1, i32::MAX) => i32::MIN;
        };

        eval_sub[Sub; (Int, Int) => Int] = {
            (1, 1) => 0;
            (1, -1) => 2;
            (-1, -1) => 0;
            (-1, i32::MAX) => i32::MIN;
        };

        eval_mul[Mul; (Int, Int) => Int] = {
            (2, 1) => 2;
            (1, -1) => -1;
            (-1, -1) => 1;
            (2, i32::MAX) => -2;
        };

        eval_div[Div; (Int, Int) => Int] = {
            (2, 2) => 1;
            (1, -1) => -1;
            (-1, -1) => 1;
            (2, i32::MAX) => 0;
        };

        eval_rem[Rem; (Int, Int) => Int] = {
            (0, 2) => 0;
            (1, 2) => 1;
            (2, 2) => 0;
            (3, 2) => 1;
        };

        eval_eq_int[Eq; (Int, Int) => Bool] = {
            (1, 1) => true;
            (1, 2) => false;
        };

        eval_eq_bool[Eq; (Bool, Bool) => Bool] = {
            (true, true) => true;
            (true, false) => false;
        };

        eval_neq_int[Neq; (Int, Int) => Bool] = {
            (1, 1) => false;
            (1, 2) => true;
        };

        eval_neq_bool[Neq; (Bool, Bool) => Bool] = {
            (true, true) => false;
            (true, false) => true;
        };

        eval_lt[Lt; (Int, Int) => Bool] = {
            (1, 1) => false;
            (2, 1) => false;
            (1, 2) => true;
        };

        eval_gt[Gt; (Int, Int) => Bool] = {
            (1, 1) => false;
            (2, 1) => true;
            (1, 2) => false;
        };

        eval_lte[Lte; (Int, Int) => Bool] = {
            (1, 1) => true;
            (2, 1) => false;
            (1, 2) => true;
        };

        eval_gte[Gte; (Int, Int) => Bool] = {
            (1, 1) => true;
            (2, 1) => true;
            (1, 2) => false;
        };

        eval_or[Or; (Bool, Bool) => Bool] = {
            (false, false) => false;
            (false, true) => true;
            (true, false) => true;
            (true, true) => true;
        };

        eval_and[And; (Bool, Bool) => Bool] = {
            (false, false) => false;
            (false, true) => false;
            (true, false) => false;
            (true, true) => true;
        };
    }
}
