use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    println!("{}",width);

    let mut str_n = String::new();
    

    io::stdin().read_line(&mut str_n).expect("error");

    println!("the str is: \"{}\" ",str_n);

    let s_number=rand::thread_rng().gen_range(1..1100);
    println!("The secret number is: {}", s_number);
}
