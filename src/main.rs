fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
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
    let n = std::env::args()
        .nth(1)
        .expect("Usage: fib [n]")
        .parse()
        .expect("Could not parse n");
    println!("{}", fib(n));
}
