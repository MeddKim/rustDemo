//! 模块注释

//=========================1.7.1 注释======================
/// 文档注释
fn main() {
//    // 行注释
//    println!("Hello, world!");
//    // if语句是表达式
//    let x = 5;
//    let y = if x == 5 { 10 } else { 15 }; // y: i32
//    println!("{}",y);
//
//    let arr = [11,12,13,14];
//    for (index,var) in arr.iter().enumerate()  {
//        println!("index = {} and var = {}", index, var);
//    }
    //------------------------------------------
    //具名结构体
//    struct A{
//        att1:i32,
//        att2:String,
//    }
    //元组类型结构体（作是一个有名字的元组，具体使用方法和一般的元组基本类似）
//    struct B(i32,u6,bool);
    //空结构体
//    struct  D;

    let peter = Person::new("Peter");
    peter.greeting();
}


//结构体使用——实现结构体
struct Person{
    name: String,
}
impl Person{
    fn new(n: &str) -> Person{
        Person{
            name: n.to_string(),
        }
    }
    fn greeting(&self){
        println!("Hello, {}!",self.name)
    }
}


/// 1. 结构体里面的引用字段必须要有显示的生命周期（非引用字段生命周期不会超过这个结构体）
/// 2. 一个被显式写出生命周期的结构体，其自身的生命周期一定小于等于其显式写出的任意一个生命周期
///
///