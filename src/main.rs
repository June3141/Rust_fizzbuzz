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
        println!(
            "{}",
            match (i % prime_1, i % prime_2, primal::is_prime(i)) {
                (0, 0, _) => "FizzBuzz",
                (0, _, _) => "Fizz",
                (_, 0, _) => "Buzz",
                (_, _, true) => "Prime",
                _ => {s = i.to_string(); &s},
            }
        );
    }
}
