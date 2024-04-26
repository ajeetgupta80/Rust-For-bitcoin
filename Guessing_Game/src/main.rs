use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number: ");
    let Secret_Num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess.");

        let mut guess = String::new(); // function string creating new instance

        io::stdin()
            .read_line(&mut guess)
            .expect("something wrong old sport");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {guess}");

        match guess.cmp(&Secret_Num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big i dont mean that"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
