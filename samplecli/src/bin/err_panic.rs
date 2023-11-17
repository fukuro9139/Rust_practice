fn get_int_from_file() -> Result<i32, MyError> {
	let path = "number.txt";

	let num_str = std::fs::read_to_string(path).map_err(|e| MyError::IO(e))?;
	num_str
		.trim()
		.parse::<i32>()
		.map(|t| t * 2)
		.map_err(|e| MyError::Num(e))
}

fn main() {
	match get_int_from_file() {
		Ok(x) => println!("{}", x),
		Err(e) => match e {
			MyError::IO(cause) => println!("IO Error: {}", cause),
			MyError::Num(cause) => println!("Parse Error: {}", cause),
		},
	}
}

enum MyError {
	IO(std::io::Error),
	Num(std::num::ParseIntError),
}