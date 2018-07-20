use std::io;

fn read_input() -> io::Result<()>{
    let mut input = String::new();

    try!(io::stdin().read_line(&mut input));

    println!("You typed: {}",input.trim());

    Ok(())
}

fn main() {
    //===============1.5.9 输入输出流=======================
//    let str = read_input();


}
