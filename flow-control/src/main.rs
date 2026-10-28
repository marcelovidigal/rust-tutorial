// flow control

// main function
fn main() {
	// if/else
	print!("\n\n### if/else\n\n");

	let n = 5;

	if n < 0 {
		print!("{} is negative", n);
	} else if n > 0 {
		print!("{} is positive", n);
	} else {
		print!("{} is zero", n);
	}

	let big_n =
		if n < 10 && n > -10 {
			println!(", and is a small number, increase ten-fold");

			// this expression returns an `i32`.
			10 * n
		} else {
			println!(", and is a big number, halve the number");

			// this expression must return an `i32` as well
			n / 2
			// todo ^ try suppressing this expression with a semicolon
		};
	//   ^ do not forget to put a semicolon here, all `let` bindings need it

	println!("{} -> {}", n, big_n);

	// loop
	print!("\n\n### loop\n\n");

	let mut count = 0u32;

	println!("count until infinity...");

	// infinite loop
	loop {
		count += 1;

		if count == 3 {
			println!("three");

			// skip the rest of this iteration
			continue;
		}

		println!("{}", count);

		if count == 5 {
			println!("ok, that is enough");

			// exit this loop
			break;
		}
	}
}