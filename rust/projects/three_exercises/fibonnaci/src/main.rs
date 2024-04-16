use std::io;
fn main() {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Not valid");
    let num =  input_str.trim().parse().expect("Not a num");
    let fib = fibonnaci(num);
    println!("{fib}");
}
fn fibonnaci(n:u32) -> u32{
    if n==1 {
        1
    }
    else if n==2 {
        1
    }
    else {
        fibonnaci(n-1)+fibonnaci(n-2)
    }
}
