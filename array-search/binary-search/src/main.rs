use common;
use std::io;
use std::io::Write;

fn bin_search(vec: & Vec<i32>, element_to_be_searched: i32, low: usize, high: usize) -> bool {
	if low <= high {
		let mid = low + (high-low)/2;
		if vec[mid] == element_to_be_searched {
			return true;
		} else if vec[mid] > element_to_be_searched {
			return bin_search(vec, element_to_be_searched, low, mid-1);
		} else {
			return bin_search(vec, element_to_be_searched, mid+1, high);
		}
	}

	return false;
}

fn search(vec: & Vec<i32>, element_to_be_searched: i32) -> bool {
	return bin_search(vec, element_to_be_searched, 0 as usize, (vec.len()-1));
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
