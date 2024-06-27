use std::io;
use Encrypt_practice::encryptor::{rot13, Encryptable};

fn main() {
    let mut user_input = String::new();

    println!("Enter string to encrypt.");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input.");

    println!("Your encrypted string: {}",
             rot13::Rot13(user_input).encrypt()
    );
}
