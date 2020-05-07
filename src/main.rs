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
        let fizzbuzz: &str =
                match (i % prime_1, i % prime_2) {
                    (0, 0) => "Fizz Buzz",
                    (0, _) => "Fizz",
                    (_, 0) => "Buzz",
                    _ => "",
                };

        let is_prime: bool  = primal::is_prime(i);


        match (fizzbuzz.len(), is_prime) {
            (0, false) => println!("{}", i.to_string()),
            (_, false) => println!("{}", fizzbuzz),
            (0, true) => println!("Prime"),
            _ => println!("{}", format!("{} Prime", fizzbuzz.to_string()))
        }
    }
}
