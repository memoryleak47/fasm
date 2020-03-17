use std::env;

use libfasm::Term;

fn main() {
	let code = include_str!("code.fasm");
	let m = Term::parse_termstr(code);
	let n = Term::parse_str(&env::args().nth(1).unwrap());
	let t = Term::Application(Box::new(m), Box::new(n));
	let t = t.execute();
	println!("{}", t.to_string());
}
