use std::io;
use std::io::Write;

fn main() {
	let mut num_of_elements: String = String::new();
	let mut arr: Vec<i32> = Vec::new();

	print!("Enter number of elements in the array: ");
	io::stdout().flush();

	io::stdin().read_line(&mut num_of_elements).expect("Failed to read line");
	let num_of_elements = num_of_elements.trim().parse::<i32>().unwrap();
	let mut element: i32;

	for i in 1..num_of_elements+1 {
		print!("Enter the value of element {:?}: ", i );
		io::stdout().flush();

		let mut num_of_elements = String::new();
		io::stdin().read_line(&mut num_of_elements).expect("Failed to read line");
		element = num_of_elements.trim().parse::<i32>().unwrap();

		arr.push(element);
	}

	print!("Enter the element to be searched in the array: ");
	io::stdout().flush();

	let mut element_to_be_searched = String::new();
	io::stdin().read_line(&mut element_to_be_searched).expect("Failed to read line");
	let element_to_be_searched = element_to_be_searched.trim().parse::<i32>().unwrap();

	let mut found = false;

	for elem in arr.iter() {
		if element_to_be_searched == *elem {
			found = true;
			break;
		}
	}

	if found == true {
		println!("Found");
	} else {
		print!("Not found");
	}
}