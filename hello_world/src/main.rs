extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("this secret_number is: {}",secret_number );

    loop {
	    println!("Please input you guess.");

	    // let创建变量绑定（默认不可变），mut让绑定可变
	    let mut guess = String::new();

	    //std::io::stdin()返回一个指向终端标准输入的句柄
	    //read_line()返回io::Result
	    io::stdin().read_line(&mut guess).expect("Failed to read line");


	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => continue,
	    };
	    	

	    println!("You guessed :{}", guess);

	    match guess.cmp(&secret_number) {
	    	Ordering::Less => println!("Too small!"),
	    	Ordering::Greater => println!("Too big"),
	    	Ordering::Equal => {
	    		println!("You win");
	    		break;
	    	} 
	    }
    }
}