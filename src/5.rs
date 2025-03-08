let mut rng = rand::thread_rng();
let number1 = rng.gen_range(0..=10);
let number2 = rng.gen_range(0..=10);
let sum = number1 + number2;
println!("The sum of {} and {} is:", number1, number2);
