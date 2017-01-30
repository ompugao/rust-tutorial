extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_num = rand::thread_rng().gen_range(1,101);
    println!("{}", secret_num);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {println!("equal"); break;}
        }
    }
}
