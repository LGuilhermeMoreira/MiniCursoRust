use std::{cmp::Ordering, io};
use rand::Rng;

fn generate_num() -> i32{
    let i = rand::thread_rng().gen_range(0..=100);
    return i;
}

fn main() {
    let num = generate_num();
    println!("generated number {}",num);
    loop {
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line!");
        
        print!("you guessed: {}",guess);
        
        
        let guess : i32 = match guess.trim()
        .parse(){
            Ok(num) => num,
            Err(e) => {
                print!("Erro ao fazer o parse da entrada, {}",e );
                return;
            }
        };
        
        match guess.cmp(&num) {
            Ordering::Less => println!("pequeno, não deu bom!"),
            Ordering::Equal => {
                println!("Igual, deu muito bom!");
                break;
            },
            Ordering::Greater => println!("maior, não deu bom")
        }
    }
}
