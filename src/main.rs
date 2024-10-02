mod simple_maths;
mod io_utils;

fn greet() {
    println!("Welcome to the prime number checker!");
    println!("Input a number to check if it is prime:");
}

fn get_user_input() -> u64 {
    let input = io_utils::read_user_input();

    input.trim().parse::<u64>().unwrap()
}

fn process_input(number: u64) -> (bool, Vec<u64>) {
    let (is_prime, divisors) = simple_maths::is_prime(number);

    (is_prime, divisors)
}

fn display_result(number: u64, is_prime: bool, divisors: Vec<u64>) {
    match is_prime {
        true => {
            println!("Your number {number} is prime.");

        }
        false => {
            println!("The number {number} is not prime. It is divisible by {divisors:?}");
        }
    }
}

fn finish() {
    println!("Goodbye!");
}

fn main() {
    greet();
    let number = get_user_input();
    let (is_prime, divisors) = process_input(number);
    display_result(number, is_prime, divisors);
    finish();
}
