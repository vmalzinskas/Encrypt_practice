pub mod encryptor;

use std::io;
use encryptor::Encryptable;

fn main() {
    let mut user_input = String::new();

    println!("Enter string to encrypt.");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input.");

    println!("Your encrypted string: {}", encryptor::rot13::Rot13(user_input).encrypt());
}
