use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    let opts = Opts::parse();

	if let Some(path) = opts.formula_file{
		let f = File::open(path).unwrap();
		let reader = BufReader::new(f);
		run(reader, opts.verbose);
	}else{
		println!("No formula");
	}
}

fn run(reader: BufReader<File>, verbose: bool){
	for line in reader.lines(){
		let line = line.unwrap();
		println!("{}", line);
	}
}
