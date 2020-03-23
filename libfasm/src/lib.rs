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

fn parse_line(s: &str) -> (Function, Pattern, Term) {
	unimplemented!()
}

impl Program {
	pub fn parse(s: &str) -> Program {
		let mut program = Program { map: HashMap::new() };
		for line in s.split(";") {
			let (func, pattern, term) = parse_line(line);
			if program.map.get(&func).is_none() {
				program.map.insert(func.clone(), Vec::new());
			}

			program.map.get_mut(&func).unwrap().push((pattern, term));
		}
		program
	}
}
