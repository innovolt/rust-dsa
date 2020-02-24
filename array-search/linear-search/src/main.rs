use common;
use std::io;
use std::io::Write;

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

	let arr = common::input_vector(var);

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