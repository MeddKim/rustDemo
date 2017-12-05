
fn main() {
//********
	// let a:i32 = 23;
	// let b:i16 = -16;

	//	let c:u16 = -18;

	// println!("{}",a);
	// println!("{}",b);
//********
	// let mut x = 10;
	// {
	// 	let y = &mut x;
	// 	*y += 1;
	// }
	// println!("{}",x );
//*********
	let li = "hello";
	let lang = "world";

	let copy_str = skip_prefix(&li,&lang);
	println!("{},{}",lang, li);

	 
	// let str = "hello,world";
	// let b = str;
	// println!("{}",str);

	// let mut test:i32 = 13;
	// let mut test2:i32 = 24;

	// num_pre(test,test2);
	// num_mut(&mut test,&mut test2);
}

fn skip_prefix(line: &str, prefix: &str) -> &'static str{
	println!("{},{}",line, prefix);
}


fn num_pre(x: i32,y: i32){
	println!("{},{}", x, y);
}

fn num_mut(x: &mut i32,y: &mut i32){
	println!("{},{}", x, y);
}
