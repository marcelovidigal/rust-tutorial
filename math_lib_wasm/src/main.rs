use std::io;
use colored::Colorize;
use math_lib::math::operations;

fn main() {
	println!("rust math cli");
	
	println!("{}", "success!".green());
	println!("{}", "warning!".yellow());
	println!("{}", "error!".red().bold());

	println!("choose an operation: add, subtract, multiply, divide");
	
	let mut operation = String::new();
	io::stdin().read_line(&mut operation).unwrap();

	let operation = operation.trim();
	let mut input1 = String::new();
	let mut input2 = String::new();
	
	println!("enter first number:");
	io::stdin().read_line(&mut input1).unwrap();
	
	println!("enter second number:");
	io::stdin().read_line(&mut input2).unwrap();
	
	let a: f64 = input1.trim().parse().unwrap();
	let b: f64 = input2.trim().parse().unwrap();
	let result = match operation {
		"add" => Some(operations::add(a, b)),
		"subtract" => Some(operations::subtract(a, b)),
		"multiply" => Some(operations::multiply(a, b)),
		"divide" => operations::divide(a, b),
		_ => {
			println!("unknown operation");
			None
		}
	};
	
	match result {
		Some(r) => println!("result: {}", r),
		None => println!("could not compute result (maybe division by zero?)"),
	}
}