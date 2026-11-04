pub fn add(a: f64, b: f64) -> f64 {
	a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
	a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
	a * b
}

pub fn divide(a: f64, b: f64) -> Option<f64> {
	if b == 0.0 {
		None
	} else {
		Some(a / b)
	}
}