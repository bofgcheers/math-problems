use std::{cmp::Ordering, io};

fn main() {
    let mut numbers = (1..=100).collect::<Vec<_>>();
    let mut random_index = 0;
    loop {
        match numbers.get(random_index) {
            Some(number) => {
                println!("Your number is: {}", number);
                break;
            }
            None => {
                random_index += 1;
                if random_index == numbers.len() {
                    numbers = (1..=100).collect::<Vec<_>>();
                    random_index = 0;
                }
            }
        }
    }
}
