use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Gues the number!!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess!");


        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("FAILED");
        //test comment

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("TOO SMALL"),
            Ordering::Greater => println!("TOO BIG"),
            Ordering::Equal => {
                println!("CORRECT"); 
                break;
            }

        }
    }

}
