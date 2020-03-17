#[derive(PartialEq, Debug, Clone)]
pub enum Term {
	Var(String), // x
	Abstraction(String, Box<Term>), // \x.M
	Application(Box<Term>, Box<Term>), // (M)(N)
}

impl Term {
	fn num(n: u8) -> Term {
		let x = "x".to_string();
		let f = "f".to_string();
		let mut term = Term::Var(x.clone());

		for _ in 0..n {
			term = Term::Application(Box::new(Term::Var(f.clone())), Box::new(term));
		}
		Term::Abstraction(f.clone(), Box::new(Term::Abstraction(x.clone(), Box::new(term))))
	}

	pub fn parse_str(x: &str) -> Term {
		assert!(x.chars().all(|c| c.is_ascii()));
		let bytes = x.as_bytes();
		let mut term = Term::num(0);
		for (i, b) in bytes.iter().rev().enumerate() {
			let t = Term::Application(Box::new(Term::Var(i.to_string())), Box::new(Term::num(*b)));
			term = Term::Application(Box::new(t), Box::new(term));
			term = Term::Abstraction(i.to_string(), Box::new(term));
		}
		term
	}

	fn insert(&self, v: &str, t: &Term) -> Term {
		match self {
			Term::Var(x) if x.as_str() == v => Term::Var(v.to_string()),
			Term::Var(x) => Term::Var(x.to_string()),
			Term::Abstraction(x, y) if x.as_str() == v => Term::Abstraction(x.to_string(), y.clone()),
			Term::Abstraction(x, y) => Term::Abstraction(x.to_string(), Box::new(y.insert(v, t))),
			Term::Application(x, y) => Term::Application(Box::new((*x).insert(v, t)), Box::new((*y).insert(v, t))),
		}
	}

	pub fn execute(&self) -> Term {
		match self {
			Term::Application(x, y) => {
				if let Term::Abstraction(v, t) = &**x {
					t.insert(&v, y).execute()
				} else {
					Term::Application(Box::new(x.execute()), Box::new(y.execute()))
				}
			},
			x => x.clone(),
		}
	}

	pub fn to_termstring(&self) -> String {
		match self {
			Term::Abstraction(x, y) => format!("\\{}.{}", &x, (*y).to_termstring()),
			Term::Application(x, y) => format!("({})({})", (*x).to_termstring(), (*y).to_termstring()),
			Term::Var(x) => x.clone(),
		}
	}

	pub fn to_string(&self) -> String {
		panic!("TODO")
	}
}
