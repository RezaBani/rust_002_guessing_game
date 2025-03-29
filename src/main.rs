fn main() {
    
    let secret_number = 5;
    
    loop {
        println!("Guess the number!");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed To read the input");

        let guess:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Guess too small"),
            std::cmp::Ordering::Equal => {
                println!("Guess correct"); 
                break;
            }
            std::cmp::Ordering::Greater => println!("Guess too large"),
        }
    }
}
