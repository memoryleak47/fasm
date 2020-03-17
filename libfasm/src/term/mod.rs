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
			Term::Var(x) if x.as_str() == v => t.clone(),
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

	fn to_num(&self) -> u8 {
		if let Term::Abstraction(f, tmp1) = self {
		if let Term::Abstraction(x, term_box) = &**tmp1 {
			let mut term: &Term = &**term_box;
			let mut n = 0;
			while let Term::Application(v, term2) = term {
				if **v == Term::Var(f.to_string()) {
					n += 1;
					term = &*term2;
				} else {
					panic!("malformed non-number term")
				}
			}
			if *term == Term::Var(x.to_string()) {
				return n;
			}
		}}
		panic!("malformed non-number term")
	}

	pub fn to_string(&self) -> String {
		// TODO make this readable for humans
		if let Term::Abstraction(v, tmp1) = self {
			if let Term::Application(tmp2, string_box) = &**tmp1 {
			if let Term::Application(v_box, char_box) = &**tmp2 {
			if Term::Var(v.to_string()) == **v_box {
				return format!("{}{}", char::from((&**char_box).to_num()), (**string_box).to_string());
			}}}

			if let Term::Abstraction(v2, tmp2) = &**tmp1 { // parse c0
			if Term::Var(v2.to_string()) == **tmp2 {
				return String::new();
			}}
		}
		panic!("malformed non-string term")
	}
}

#[test]
fn test_to_termstring() {
	let i = Term::Abstraction("x".to_string(), Box::new(Term::Var("x".to_string())));
	assert_eq!("\\x.x", &i.to_termstring());
}

#[test]
fn test_execute() {
	let ix = Term::Abstraction("x".to_string(), Box::new(Term::Var("x".to_string())));
	let iy = Term::Abstraction("y".to_string(), Box::new(Term::Var("y".to_string())));
	assert_eq!(ix, Term::Application(Box::new(ix.clone()), Box::new(ix.clone())).execute());
	assert_eq!(iy, Term::Application(Box::new(ix.clone()), Box::new(iy.clone())).execute());
}

#[test]
fn test_numbers() {
	let n = 5;
	assert_eq!(Term::num(n).to_num(), n);
}

#[test]
fn test_strings() {
	let s = "woo";
	assert_eq!(Term::parse_str(s).to_string(), s);
}
