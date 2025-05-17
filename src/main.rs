use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Guess the number game!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop{
            println!("Please input your guess: ");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Error occured");

    println!("you guessed: {}",guess);
    let guess:u32 = guess.trim().parse().expect("Please type a number!");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("you win!");
            break;
        }
    }
    }


    // let x:u8=254;
    // let y:u8=1;

    // println!("x={} and y={} and x+y={}",x,y,x+y);


}