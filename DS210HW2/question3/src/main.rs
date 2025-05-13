use std::io;
fn sumcube(k: u8) -> u32 {
    let mut sum = 0;
    for i in 1..=k as u32 {
        sum += i.pow(3);
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let k: u8 = input.parse().expect("Not a good number!");

    println!("{}", sumcube(k));
}