use std::io;

fn main() {
    println!("Please enter your username: ");
    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("error: unable to read user input");

    println!("Welcome {}", username);
}
