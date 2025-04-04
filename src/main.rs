use std::{cmp, env, fmt::Debug, io, process::Command, str::FromStr};

fn main() {
    type NumberType = u8;
    let secret_number = generate_random_number::<NumberType>();

    println!("Guess the number! (Type q to quit eartly)");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To read the input");

        let guess: NumberType = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim().to_lowercase() == "q" {
                    break;
                }
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Guess too small"),
            cmp::Ordering::Equal => {
                println!("Guess correct");
                break;
            }
            cmp::Ordering::Greater => println!("Guess too large"),
        }
    }
}

fn generate_random_number<T: std::str::FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let rust_cwd = env::current_dir().unwrap();
    env::set_current_dir("../../Zig/zig_004_random_number_generator/").expect("Change cwd failed");
    let mut zig = Command::new("zig");
    let output = zig
        .args(vec![
            "build",
            "run",
            "--",
            format!("{}", std::any::type_name::<T>()).as_str(),
        ])
        .output()
        .unwrap();
    let number = String::from_utf8(output.stderr).unwrap();
    let number = number.trim();
    env::set_current_dir(rust_cwd).unwrap();
    number.parse().unwrap()
}
