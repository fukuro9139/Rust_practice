fn get_int_from_file() -> Result<i32, MyError> {
	let path = "number.txt";

	let num_str = std::fs::read_to_string(path).map_err(MyError::from)?;
	num_str
		.trim()
		.parse::<i32>()
		.map(|t| t * 2)
		.map_err(MyError::from)
}

fn main() {
	match get_int_from_file() {
		Ok(x) => println!("{}", x),
		Err(e) => println!("{}", e),
	}
}

enum MyError {
	IO(std::io::Error),
	Num(std::num::ParseIntError),
}

impl From<std::io::Error> for MyError {
	fn from(cause: std::io::Error) -> Self {
		MyError::IO(cause)
	}
}

impl From<std::num::ParseIntError> for MyError {
	fn from(cause: std::num::ParseIntError) -> Self {
		MyError::Num(cause)
	}
}

impl std::fmt::Display for MyError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			MyError::IO(cause) => write!(f, "IO Error: {}", cause),
			MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
		}
	}
}