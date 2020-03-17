#[derive(PartialEq, Debug)]
pub enum Term {
	Var(String), // x
	Abstraction(String, Box<Term>), // \x.M
	Application(Box<Term>, Box<Term>), // (M)(N)
}

impl Term {
	pub fn parse_str(x: &str) -> Term {
		panic!("TODO")
	}

	pub fn execute(&self) -> Term {
		panic!("TODO")
	}

	pub fn to_termstring(&self) -> String {
		panic!("TODO")
	}

	pub fn to_string(&self) -> String {
		panic!("TODO")
	}
}
