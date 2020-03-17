use std::process::Command;
use std::env;

fn main() {
	let code_file = env::args().nth(1).expect("missing file to compile");

	let tmpdir: String = String::from_utf8(Command::new("mktemp")
		.arg("-d")
		.output()
		.unwrap()
		.stdout)
		.unwrap()
		.trim()
		.to_string();
	dbg!(&tmpdir);

	let f = |c: &mut Command| println!("{}", String::from_utf8(c.output().unwrap().stderr).unwrap());

	f(Command::new("cp").arg("-r").arg("libfasm").arg(&tmpdir));
	f(Command::new("cp").arg("-r").arg("fasmexec").arg(&tmpdir));
	f(Command::new("cp").arg(code_file).arg(&format!("{}/fasmexec/src/code.fasm", &tmpdir)));
	f(Command::new("cargo").arg("b").arg("--manifest-path").arg(format!("{}/fasmexec/Cargo.toml", &tmpdir)));
	f(Command::new("cp").arg(format!("{}/fasmexec/target/debug/fasmexec", &tmpdir)).arg("a.out"));
}
