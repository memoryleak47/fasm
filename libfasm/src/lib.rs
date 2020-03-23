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

enum Token {
	Atom(Atom),
	Var(Var),
	Function(Function),
	Comma,
	LeftParen,
	RightParen,
	Equals,
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

fn parse_pattern(t: Vec<Token>) -> Pattern {
	unimplemented!()
}

fn parse_term(t: Vec<Token>) -> Term {
	unimplemented!()
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
		let mut program = Program { map: HashMap::new() };
		for line in s.split(';') {
			let (function, pattern, term) = parse_line(line);
			program.map.entry(function).or_insert(Vec::new()).push((pattern, term));
		}
		program
	}
}
