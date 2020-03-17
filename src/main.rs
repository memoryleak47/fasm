use std::fs::File;
use std::io::Write;
use std::process::Command;

mod term;
mod parse;

use term::Term;

fn main() {
	let t = Term::parse("\\x.x");
	let s = t.to_llvm_ir();

	let mut file = File::create("foo.ll").unwrap();
	file.write_all(s.as_bytes()).unwrap();

	println!("{:?}", Command::new("clang")
		.arg("foo.ll")
		.output()
		.unwrap());
}
