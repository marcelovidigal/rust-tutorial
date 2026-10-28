// this is a comment, and is ignored by the compiler

// main function.
fn main() {
	// statements here are executed when the compiled binary is called

	// print text to the console.
	println!("hello world!");

	// in general, the `{}` will be automatically replaced with any arguments
	// these will be stringified
	println!("{} days", 31);

	// positional arguments can be used
	// specifying an integer inside `{}` determines which additional argument will be replaced
	// arguments start at 0 immediately after the format string
	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

	// as can named arguments
	println!(
		"{subject} {verb} {object}",
		object="the lazy dog",
		subject="the quick brown fox",
		verb="jumps over"
	);

	// different formatting can be invoked by specifying the format character after a `:`
	println!("base 10:               {}  ", 69420); // 69420
	println!("base 2 (binary):       {:b}", 69420); // 10000111100101100
	println!("base 8 (octal):        {:o}", 69420); // 207454
	println!("base 16 (hexadecimal): {:x}", 69420); // 10f2c

	// right-justify text with a specified width this will output
	println!("{number:>5}", number=1);

	// pad numbers with extra zeroes
	println!("{number:0>5}", number=1); // 00001
	
	// left-adjust by flipping the sign, this will output "10000"
	println!("{number:0<5}", number=1); // 10000

	// use named arguments in the format specifier by appending a `$`
	println!("{number:0>width$}", number=1, width=5);

	// rust even checks to make sure the correct number of arguments are used.
	//println!("my name is {0}, {1} {0}", "bond");
	// fixme ^ add the missing argument "james"
	println!("my name is {0}, {1} {0}", "bond", "james");

	// only types that implement fmt::Display can be formatted with `{}`
	// user-defined types do not implement fmt::Display by default

	#[allow(dead_code)] // disable `dead_code` which warn against unused module
	struct Structure(i32);

	// this will not compile because `Structure` does not implement fmt::Display
	//println!("this struct `{}` won't print...", Structure(3));
	// todo ^ try uncommenting this line

	// for rust 1.58 and above, you can directly capture the argument from a surrounding variable
	// just like the above, this will output "    1", 4 white spaces and a "1"
	let number: f64 = 1.0;
	let width: usize = 5;
	println!("{number:>width$}");

}
