mod term;
mod parse;

use term::Term;

fn main() {
	dbg!(Term::parse("\\x.x"));
}
