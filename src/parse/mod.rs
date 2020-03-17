mod token;

use token::{Token, tokenize};
use crate::term::Term;

fn eat_application(tokens: &[Token]) -> (&[Token], Term) {
	assert_eq!(tokens[0], Token::LeftParen);
	let (tokens, l) = eat_term(&tokens[1..]);
	
	assert_eq!(tokens[0], Token::RightParen);
	assert_eq!(tokens[1], Token::LeftParen);

	let (tokens, r) = eat_term(&tokens[2..]);

	assert_eq!(tokens[0], Token::RightParen);
	(&tokens[1..], Term::Application(Box::new(l), Box::new(r)))
}

fn eat_abstraction(tokens: &[Token]) -> (&[Token], Term) {
	if let [Token::Lambda, Token::Var(ref x), Token::Dot, ref rest @ ..] = tokens {
		let (rest, term) = eat_term(rest);
		return (rest, Term::Abstraction(x.clone(), Box::new(term)));
	}
	panic!("syntax error")
}

fn eat_term(tokens: &[Token]) -> (&[Token], Term) {
	assert!(!tokens.is_empty());
	match tokens[0] {
		Token::LeftParen => eat_application(tokens),
		Token::Lambda => eat_abstraction(tokens),
		Token::Var(ref x) => (&tokens[1..], Term::Var(x.clone())),
		Token::Dollar => (&tokens[1..], Term::Input),
		_ => panic!("syntax error")
	}
}

impl Term {
	pub fn parse(s: &str) -> Term {
		let tokens = tokenize(s);
		if let ([], term) = eat_term(&tokens[..]) {
			return term;
		}
		panic!("syntax error")
	}
}
