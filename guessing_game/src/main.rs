use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("Secret number: {}", secret_number);

    loop {
        println!("Enter your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Wow! shadowing the old guess variable. Needed because rust is strongly typed (no implicit type casting) AND statically typed (albeit with inference, e.g. let mut guess = String::new())
        let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("error mate: {}", err);
                continue;
            },
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Well done, you win.");
                break;
            },
        }
    }
}
