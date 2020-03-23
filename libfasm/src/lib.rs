use std::collections::HashMap;

pub struct Atom(String);
pub struct Var(String);
pub struct Function(String);

pub enum Pattern {
	Atom(Atom),
	Var(Var),
	Tuple(Vec<Pattern>),
}

pub enum Term {
	Atom(Atom),
	Var(Var),
	Tuple(Vec<Term>),
	Call(Function, Box<Term>),
}

pub struct Line {
	function: Function,
	pattern: Pattern,
	output: Term,
}

pub struct Program {
	map: HashMap<Function, Vec<(Pattern, Term)>>,
}

impl Program {
	pub fn parse(s: String) -> Program {
		unimplemented!() // TODO
	}
}
