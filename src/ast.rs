// Similar to the original AST with some extra derives
// https://github.com/aripiprazole/rinha-de-compiler/blob/main/src/ast.rs

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}

impl<T: Element> Element for Rc<T> {
    fn location(&self) -> &Location {
        self.as_ref().location()
    }
}

impl<T: Element> Element for Box<T> {
    fn location(&self) -> &Location {
        self.as_ref().location()
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

impl Location {
    pub fn new(start: usize, end: usize, filename: &str) -> Self {
        Self {
            start,
            end,
            filename: filename.into(),
        }
    }

    pub fn merge(self, other: Self) -> Self {
        assert_eq!(self.filename, other.filename);
        Self {
            start: self.start,
            end: other.end,
            filename: self.filename,
        }
    }
}

pub trait Element {
    fn location(&self) -> &Location;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub full_text: String,
    pub location: Location,
}

impl Element for Error {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Let {
    pub name: Var,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

impl Element for Str {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}

impl Element for Bool {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}

impl Element for Int {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOp,
    pub rhs: Box<Term>,
    pub location: Location,
}

impl Element for Binary {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
    pub location: Location,
}

impl Element for Call {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub parameters: Vec<Var>,
    pub value: Box<Term>,
    pub location: Location,
}

impl Element for Function {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

impl Element for Print {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct First {
    pub value: Box<Term>,
    pub location: Location,
}

impl Element for First {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Second {
    pub value: Box<Term>,
    pub location: Location,
}

impl Element for Second {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tuple {
    pub first: Box<Term>,
    pub second: Box<Term>,
    pub location: Location,
}

impl Element for Tuple {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Var {
    pub text: String,
    pub location: Location,
}

impl Element for Var {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Error(Error),
    Int(Int),
    Str(Str),
    Call(Call),
    Binary(Binary),
    Function(Function),
    Let(Let),
    If(If),
    Print(Print),
    First(First),
    Second(Second),
    Bool(Bool),
    Tuple(Tuple),
    Var(Var),
}

impl Element for Term {
    fn location(&self) -> &Location {
        match self {
            Term::Error(arg0) => &arg0.location,
            Term::Int(arg0) => &arg0.location,
            Term::Str(arg0) => &arg0.location,
            Term::Function(arg0) => &arg0.location,
            Term::Call(arg0) => arg0.location(),
            Term::Var(arg0) => arg0.location(),
            Term::Binary(arg0) => &arg0.location,
            Term::Print(arg0) => &arg0.location,
            Term::First(arg0) => &arg0.location,
            Term::Second(arg0) => &arg0.location,
            Term::Let(arg0) => &arg0.location,
            Term::If(arg0) => &arg0.location,
            Term::Bool(arg0) => &arg0.location,
            Term::Tuple(arg0) => arg0.location(),
        }
    }
}
