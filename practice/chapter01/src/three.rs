fn three(){
    //===============1.5.4 结构体和枚举=======================
    //--------------------------结构体------------------------------
//    struct Point{
//        x:i32,
//        y:i32
//    }
//    let mut point = Point{x:30,y:20};
//    println!("{}---{}",point.x,point.y);
//    point.x = 99;
//    point.y = 100;
//    println!("{}---{}",point.x,point.y);
//    point = Point{x:222,y:333};
//    println!("{}---{}",point.x,point.y);
//**********************************************
//    struct Color(u8,u8,u8);
//    let android_green = Color(0xa4,0x46,0x39);
//    let Color(red,green,blue) = android_green;
//    println!("{},{},{}",red,green,blue);
//**********************************************
    #[derive(Default)]
    struct Point3d{
        x:i32,
        y:i32,
        z:i32
    }
    let origin = Point3d::default();
    let point = Point3d{y:1,..origin};
    println!("{},{},{}",point.x,point.y,point.z);

}