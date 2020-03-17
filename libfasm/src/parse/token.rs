#[derive(PartialEq, Debug)]
pub enum Token {
	Var(String),
	Lambda,
	Dot,
	LeftParen,
	RightParen,
}

pub fn tokenize(string: &str) -> Vec<Token> {
	let string = string.trim();

	let mut v = Vec::new();
	let mut s = String::new();

	for x in string.chars() {
		if x.is_alphanumeric() || x == '_' {
			s.push(x);
		} else {
			if !s.is_empty() {
				v.push(Token::Var(s));
				s = String::new();
			}
			v.push(match x {
				'\\' => Token::Lambda,
				'.' => Token::Dot,
				'(' => Token::LeftParen,
				')' => Token::RightParen,
				c => panic!("wrong symbol: {:?} ", c)
			});
		}
	}
	if !s.is_empty() {
		v.push(Token::Var(s));
	}

	v
}
