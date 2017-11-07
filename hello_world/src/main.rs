use std::io;

fn main() {
    println!("Guess the number");

    println!("Please input you guess.");

    // let创建变量绑定（默认不可变），mut让绑定可变
    let mut guess = String::new();

    //std::io::stdin()返回一个指向终端标准输入的句柄
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed :{}", guess);
}