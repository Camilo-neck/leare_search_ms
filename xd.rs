fn sumar(a: i32, b: i32) -> i32 {
	a+b
}

fn main() {
	let sum = match sumar(2,3) {
		5  => 5,
		_value => -1
	};
	println!("{}", sum)
}