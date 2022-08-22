use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    let s: u32 = rand::thread_rng().gen_range(1..=100);


    loop{
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Falid to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&s) {
            Ordering::Less => println!("You guessed lower than s"),
            Ordering::Greater => println!("You guessed Higher than s"),
            Ordering::Equal => {
                println!("Yay you got is right it eas {s}");
                break;
            },
        };

        println!("you guessed {} and s = {}", guess, s);

    }
}
