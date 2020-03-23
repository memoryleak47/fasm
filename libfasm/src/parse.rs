#![feature(str_strip)]

use crate::Program;

#[derive(PartialEq, Eq)]
enum Token {
	Atom(Atom),
	Var(Var),
	Function(Function),
	Comma,
	LeftParen,
	RightParen,
}

fn split_at_char(s: &str, c: char) -> (&str, &str) {
	let i = s.find(c).unwrap();
	(&s[0..i], &s[(i+1)..])
}

fn eat_name(s: &mut String) -> String {
	let mut tmp = String::new();
	while let Some(x) = s.chars().next() {
		if !x.is_alphanumeric() { return tmp; }
		tmp.push(s.remove(0));
	}
	return tmp;
}

fn tokenize(s: &str) -> Vec<Token> {
	let mut s = s.to_owned();
	let mut t = Vec::new();

	while s.len() > 0 {
		let c = s.remove(0);
		match c {
			',' => t.push(Token::Comma),
			'(' => t.push(Token::LeftParen),
			')' => t.push(Token::RightParen),
			'$' => t.push(Token::Var(Var(eat_name(&mut s)))),
			':' => t.push(Token::Atom(Atom(eat_name(&mut s)))),
			x if x.is_alphanumeric() => {
				s.insert(0, x); // TODO improve this
				t.push(Token::Function(Function(eat_name(&mut s))));
			},
			x if x.is_whitespace() => {},
			x => panic!("unexpected symbol: {}", x),
		}
	}

	t
}

fn decay_term_to_pattern(t: Term) -> Pattern {
	match t {
		Term::Atom(a) => Pattern::Atom(a),
		Term::Var(v) => Pattern::Var(v),
		Term::Tuple(vec) => Pattern::Tuple(vec.into_iter().map(decay_term_to_pattern).collect()),
		Term::Call(..) => panic!("patterns can't contain calls!"),
	}
}

fn parse_pattern(t: Vec<Token>) -> Pattern {
	let term = parse_term(t);
	decay_term_to_pattern(term)
}

fn parse_term(mut t: Vec<Token>) -> Term {
	if let &[ref x] = &t[..] {
		return match x {
			Token::Atom(y) => Term::Atom(y.clone()),
			Token::Var(y) => Term::Var(y.clone()),
			_ => panic!("error: malformed term"),
		};
	}
	let x = t.remove(0);
	if let Token::Function(f) = x {
		return Term::Call(f, Box::new(parse_term(t)));
	}
	if x != Token::LeftParen {
		panic!("error: malformed term");
	}

	// tuple parsing
	let mut depth = 1;
	let mut tmp = Vec::new();
	let mut subterms = Vec::new();
	while !t.is_empty() {
		let x = t.remove(0);
		if x == Token::LeftParen { depth += 1; }
		if x == Token::RightParen { depth -= 1; }
		if depth == 0 && x == Token::RightParen {
			assert!(t.is_empty());
			subterms.push(parse_term(tmp));
			return Term::Tuple(subterms);
		} else if depth == 1 && x == Token::Comma {
			subterms.push(parse_term(tmp));
			tmp = Vec::new();
		} else {
			tmp.push(x);
		}
	}
	panic!("error: malformed term")
}

fn parse_line(s: &str) -> (Function, Pattern, Term) {
	let (left, right) = split_at_char(s, '=');
	let mut left = tokenize(left);
	let right = tokenize(right);

	let function = match left.remove(0) {
		Token::Function(f) => f,
		_ => panic!("error: line didn't start with function"),
	};

	let pattern = parse_pattern(left);
	let term = parse_term(right);

	(function, pattern, term)
}

impl Program {
	pub fn parse(s: &str) -> Program {
		let s = match s.strip_suffix(';') { // this allows trailing ';'
			Some(x) => x,
			None => s,
		};

		let mut program = Program { map: HashMap::new() };
		for line in s.split(';') {
			let (function, pattern, term) = parse_line(line);
			program.map.entry(function).or_insert(Vec::new()).push((pattern, term));
		}
		program
	}
}

#[test]
fn test_() {
	Program::parse("main $x = $x; main :x = :y;");
}

