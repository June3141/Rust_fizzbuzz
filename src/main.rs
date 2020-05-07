use std::io;

fn main() {
    println!("Please input start and end.");

    let s = {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let (start, end) = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let m: i32 = ws.next().unwrap().parse().unwrap();
        (n, m)
    };


    for i in start..(end + 1) {
        let s;
        println!(
            "{}",
            match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz",
                (0, _) => "Fizz",
                (_, 0) => "Buzz",
                _ => {s = i.to_string(); &s},
            }
        );
    }
}
