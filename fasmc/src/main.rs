use std::process::Command;
use std::env;

fn main() {
	let code_file = env::args().nth(1).expect("missing command line argument");

	let tmpdir: String = String::from_utf8(Command::new("mktemp")
		.arg("-d")
		.output()
		.unwrap()
		.stdout)
		.unwrap();

	Command::new("cp").arg("-r").arg("libfasm").arg(&tmpdir).output().unwrap();
	Command::new("cp").arg("-r").arg("fasmexec").arg(&tmpdir).output().unwrap();
	Command::new("cp").arg(code_file).arg(&format!("{}/src/code.fasm", &tmpdir)).output().unwrap();
	Command::new("cargo").arg("b").arg("--manifest-path").arg(format!("{}/fasmexec", &tmpdir)).output().unwrap();
	Command::new("cp").arg(format!("{}/fasmexec/target/debug/fasmexec", &tmpdir)).arg("a.out").output().unwrap();
}

