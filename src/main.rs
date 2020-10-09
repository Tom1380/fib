fn fib(n: u64) -> u64 {
	if n < 2 {
		return n
	}
	let mut previous = 0u64;
	let mut current = 1u64;
	let mut tmp: u64;
	for _ in 1..n {
		tmp = current;
		current = current + previous;
		previous = tmp;
	}
	current
}

fn main() {
	println!("{}", fib(1_000_000_000_000));
}
