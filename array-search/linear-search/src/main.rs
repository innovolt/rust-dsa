use std::io;
use std::io::Write;

fn input_vector(number_of_elements: i32) -> Vec<i32> {
	let mut arr: Vec<i32> = Vec::new();

	for i in 1..number_of_elements+1 {
		print!("Enter the value of element {:?}: ", i );
		//io::stdout().flush()?;
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

fn search(vec: & Vec<i32>, element_to_be_searched: i32) -> bool {
	for elem in vec.iter() {
		if element_to_be_searched == *elem {
			return true;
		}
	}
	return false;
}

fn main() {
	let mut var: String = String::new();

	print!("Enter number of elements in the array: ");
	if let Err(error) = io::stdout().flush() {
		panic!("{}", error);
	}

	io::stdin().read_line(&mut var).expect("Failed to read line");
	let var = var.trim().parse::<i32>().unwrap();

	let arr = input_vector(var);

	print!("Enter the element to be searched in the array: ");
	if let Err(error) = io::stdout().flush() {
		panic!("{}", error);
	}

	let mut var = String::new();
	io::stdin().read_line(&mut var).expect("Failed to read line");
	let var = var.trim().parse::<i32>().unwrap();

	if search(&arr, var) {
		println!("Found");
	} else {
		print!("Not found");
	}
}