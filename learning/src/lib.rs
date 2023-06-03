use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number between 1 and 100");

    loop {
        let mut guess = String::new();
        println!("Please input your guess");

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(error) => panic!("Problem in reading the input: {:?}", error),
        };

        let guess = match guess.trim().parse::<u32>() {
            Ok(res) => res,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Two small"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
            Ordering::Greater => println!("Too large"),
        }
    }
}

pub fn interpolation() {
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}
