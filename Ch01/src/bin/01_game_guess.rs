extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    println!("Здогадайтесь число!");
    // thread_rng() -> ThreadRng -> gen_range -> u32
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Секретне число: {}", &secret_number);

    loop {
        println!("Будь-ласка введіть ваше припущення.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // ковертуємо String в u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваше припущення: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Занадто мало!"),
            Ordering::Greater => println!("Занадто багато!"),
            Ordering::Equal => {
                println!("Ви виграли!");
                break;
            }
        }
    }
}