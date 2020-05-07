extern crate primal;

use std::io;

fn read_nums_pair() -> (u64, u64) {
    let s = {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let mut ws = s.split_whitespace();
    let n: u64 = ws.next().unwrap().parse().unwrap();
    let m: u64 = ws.next().unwrap().parse().unwrap();
    (n, m)
}

fn main() {
    println!("Please input start and end.");
    let (start, end) = read_nums_pair();

    println!("Please input two primes.");
    let (prime_1, prime_2) = read_nums_pair();

    for i in start..(end + 1) {
        let s;

        if primal::is_prime(i) {
            println!(
                "{}",
                match (i % prime_1, i % prime_2) {
                    (0, 0) => "Fizz Buzz Prime",
                    (0, _) => "Fizz Prime",
                    (_, 0) => "Buzz Prime",
                    _ => "Prime",
                }
            );
        } else {
            println!(
                "{}",
                match (i % prime_1, i % prime_2) {
                    (0, 0) => "Fizz Buzz",
                    (0, _) => "Fizz",
                    (_, 0) => "Buzz",
                    _ => {s = i.to_string(); &s},
                }
            );
        }
    }
}
