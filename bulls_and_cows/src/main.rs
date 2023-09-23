use rand::Rng;
use std::io;
use std::cmp;

fn main() {
    let mut secret_number = [0; 4];
    for index in 0..4 {
        secret_number[index] = rand::thread_rng().gen_range(0..10);
    }

    let mut input_count = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess.len() != 4 {
            continue;
        };

        input_count += 1;

        let mut guess_chars = guess.chars();

        let mut guess_number = [0; 4];
        for index in 0..4 {
            guess_number[index] = guess_chars.next().unwrap().to_digit(10).unwrap();
        }

        let mut snc = [0; 10];
        let mut gnc = [0; 10];

        for index in 0..4 {
            snc[secret_number[index]] += 1; 
            gnc[guess_number[index] as usize] += 1;
        }

        let mut b = 0;
        for index in 0..10 {
            b += cmp::min(snc[index],gnc[index]);
        }

        let mut a = 0;
        for index in 0..4 {
            if secret_number[index] as u32 == guess_number[index] {
                a += 1;
                b -= 1;
            } 
        }

        println!("A : {}, B : {}", a, b);

        if a == 4 {
            println!("You Win! You tried : {}", input_count);
            break;
        }
    }
}
