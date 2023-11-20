use anyhow::{Context, Result, ensure};

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

#[derive(Parser, Debug)]
#[clap(
	name = "My RPN program",
	version = "1.0.0",
	author = "Kengo Fukunaga",
	about = "Super awesome sample RPN calculator"
)]
struct Opts {
	/// Sets the level of verbosity
	#[clap(short, long)]
	verbose: bool,

	/// Formulas written in RPN
	#[clap(name = "FILE")]
	formula_file: Option<String>,
}

struct RpnCalculator(bool);

impl RpnCalculator {
	pub fn new(verbose: bool) -> Self {
		Self(verbose)
	}

	pub fn eval(&self, formula: &str) -> Result<i32> {
		let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
		self.eval_formula(&mut tokens)
	}

	fn eval_formula(&self, tokens: &mut Vec<&str>) -> Result<i32> {
		let mut stack = Vec::new();
		let mut pos=0;
		while let Some(token) = tokens.pop(){
			pos += 1;
			if let Ok(x) = token.parse::<i32>(){
				stack.push(x);
			}else{
				let y = stack.pop().context(format!("invaild syntax at {}", pos))?;
				let x = stack.pop().context(format!("invaild syntax at {}", pos))?;
				let res = match token {
					"+" => x + y,
					"-" => x - y,
					"*" => x * y,
					"/" => x / y,
					"%" => x % y,
					_ => panic!("invalid token"),
				};
				stack.push(res);
			}
			if self.0 {
				println!("{:?} {:?}", tokens, stack);
			}
		}

		ensure!(stack.len() == 1, "invaild syntax");
		Ok(stack[0])

	}
}

fn main() {
    let opts = Opts::parse();

	if let Some(path) = opts.formula_file{
		let f = File::open(path).unwrap();
		let reader = BufReader::new(f);
		let _ = run(reader, opts.verbose);
	}else{
		let stdin = stdin();
		let reader = stdin.lock();
		let _ = run(reader, opts.verbose);
	}
}

fn run<R:BufRead>(reader: R, verbose: bool) -> Result<()> {
	let calc = RpnCalculator::new(verbose);
	
	for line in reader.lines(){
		let line = line?;
		match calc.eval(&line){
			Ok(answer) => println!("{}", answer),
			Err(e) => eprintln!("{:#?}", e),
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_ok(){
		let calc = RpnCalculator::new(false);
		assert_eq!(calc.eval("5").unwrap(), 5);
		assert_eq!(calc.eval("50").unwrap(), 50);
		assert_eq!(calc.eval("-50").unwrap(), -50);

		assert_eq!(calc.eval("2 3 +").unwrap(), 5);
		assert_eq!(calc.eval("2 3 -").unwrap(), -1);
		assert_eq!(calc.eval("2 3 *").unwrap(), 6);
		assert_eq!(calc.eval("2 3 /").unwrap(), 0);
		assert_eq!(calc.eval("2 3 %").unwrap(), 2);
	}

	#[test]
	#[should_panic]
	fn test_ng(){
		let calc = RpnCalculator::new(false);
		assert!(calc.eval("").is_err());
		assert!(calc.eval("1 1 +").is_err());
		assert!(calc.eval("+ 1 1").is_err());
	}
}