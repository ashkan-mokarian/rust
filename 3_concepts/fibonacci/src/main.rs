// Return the n-th fibonacci number
use std::io;

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    println!("Fib({}) = {}", n, fibonacci(n));
}
