fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }
    let mut a = 1;
    let mut b = 0;
    let mut sum: i32;
    for _ in 2..n {
        sum = a + b;
        b = a;
        a = sum;
    }
    a
}

fn main() {
    println!("{}C = {}F", 0, c_to_f(0.0));
    println!("{}F = {}C", 100, f_to_c(100.0));
    println!("The {}th fibonnaci number is {}", 8, fib(8));
}
