use std::io;
use std::io::Write;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// It constructs a vector of size number_of_elements
pub fn input_vector(number_of_elements: i32) -> Vec<i32> {
	let mut arr: Vec<i32> = Vec::new();

	for i in 1..number_of_elements+1 {
		print!("Enter the value of element {:?}: ", i );
		if let Err(error) = io::stdout().flush() {
			panic!("{}", error);
		}

		let mut element = String::new();
		io::stdin().read_line(&mut element).expect("Failed to read line");
		let element = element.trim().parse::<i32>().unwrap();

		arr.push(element);
	}

	return arr;
}