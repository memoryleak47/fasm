#![feature(str_strip)]

use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Atom(String);

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Var(String);

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Function(String);

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Pattern {
	Atom(Atom),
	Var(Var),
	Tuple(Vec<Pattern>),
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Term {
	Atom(Atom),
	Var(Var),
	Tuple(Vec<Term>),
	Call(Function, Box<Term>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Program {
	map: HashMap<Function, Vec<(Pattern, Term)>>,
}
