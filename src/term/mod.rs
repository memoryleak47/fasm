#[derive(PartialEq, Debug)]
pub enum Term {
	Var(String), // x
	Abstraction(String, Box<Term>), // \x.M
	Application(Box<Term>, Box<Term>), // (M)(N)
}
