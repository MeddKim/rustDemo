fn four(){
    //===============1.5.5 控制流=======================
    //--------------------- if --------------------------
//    let x = 5;
//    let y = if x == 5{ 10 }else { 15 };
//    println!("{}",y)

    //rust中的两种语句
    //      声明语句：比如进行变量绑定的let语句
    //      表达式语句：用过在末尾加上分号; 来将表达式变成语句


    //-------------------- for ---------------------------
//    let a = [1,2,3,4,5];
//    for var in a.iter(){
//        println!("{}",var)
//    }
    //-------------------- while -------------------------
    //-------------------- Match -------------------------
//    let day = 5;
//    match day {
//        0 | 6 => println!("weekend"),
//        1 ... 5 => println!("weekday"),
//        _ => println!("invalid"),
//    }
    //***********************************
    // 是用 @ 绑定匹配到的变量
//    let x = 3;
//    match x {
//        e @ 1...5 => println!("got a range element {}",e),
//        _=> println!("anything")
//    }
    //***********************************
    //  使用 ref 得到一个引用
    let x = 5;
    let mut y = 5;

    match x {
        ref r => println!("Got a refrence to {}", r)
    }
    match y {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }
}