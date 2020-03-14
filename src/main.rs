use std::io;

fn main() {
    let mut number = String::new();
    println!("Enter you number");
    io::stdin().read_line(&mut number).expect("Uknown format");
    let number: u64 = number.trim().parse().expect("Please type a number!");

    let fibo = fib(number);
    println!("The fib is {}", fibo);
}

fn fib(n: u64) -> u64 {
    assert!(n <= 75);
    let golden = (1.0 + 5.0f64.sqrt()) / 2.0;
    let golden_n = golden.powi(n as i32);
    let approx = golden_n * 0.2f64.sqrt();
    let exact = approx.round();
    exact as u64
}
