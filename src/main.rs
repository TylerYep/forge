use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub mod algorithms;
pub mod structures;

fn main() {
    let arr = vec![0, 2, 4, 5];
    let result = algorithms::binary_search(&2, &arr).unwrap();
    println!("{result:?}");
    let q = queue![17, 1, 5];
    let answer = q.peek().unwrap();
    println!("{answer}");
    guessing_game();
}

fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Guess the number!");
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => break,
        };
    }
}
