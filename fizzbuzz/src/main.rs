use fizzy_bubblech::{fizzbuzz, call_correct};

fn main() {
    println!("Basic implementation.");
    fizzbuzz_easy(5);
    fizzbuzz_easy(3);
    fizzbuzz_easy(1);
    fizzbuzz_easy(15);
    println!("More interesting implementation using enums.");
    call_correct(fizzbuzz(1));
    call_correct(fizzbuzz(2));
    call_correct(fizzbuzz(3));
    call_correct(fizzbuzz(4));
    call_correct(fizzbuzz(5));
    call_correct(fizzbuzz(6));
    call_correct(fizzbuzz(7));
    call_correct(fizzbuzz(8));
    call_correct(fizzbuzz(9));
    call_correct(fizzbuzz(10));
    call_correct(fizzbuzz(15));
}

fn fizzbuzz_easy(number: i32) -> () {
    if number % 3 == 0 {
        println!("Fizz");
    } else if number % 5 == 0 {
        println!("Buzz");
    } else if number % 15 == 0 {
        println!("FizzBuzz");
    } else {
        println!("{}", number);
    };
}

// The following function and enum makes use of Rust enums, which can hold values of different types!
mod fizzy_bubblech {
    pub fn fizzbuzz(number: i32) -> Answer {
        if number % 3 == 0 {
            Answer::Word(String::from("Fizz"))
        } else if number % 5 == 0 {
            Answer::Word(String::from("Buzz"))
        } else if number % 15 == 0 {
            Answer::Word(String::from("FizzBuzz"))
        } else {
            Answer::Equality(number)
        }
    }
    
    pub type Answer = FizzBuzzAnswer;
    
    pub enum FizzBuzzAnswer {
        Word(String),
        Equality(i32),
    }

    pub fn call_correct(result: Answer) {
        match result {
            Answer::Word(res) => println!("{}", res),
            Answer::Equality(num) => println!("{}", num),
        }
    }
}