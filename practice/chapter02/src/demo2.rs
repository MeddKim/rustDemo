trait HasArea{
    fn area(&self) -> f64;
}
struct Circle{
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle{
    fn area(&self) -> f64{
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let c = Circle{
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };
    println!("{}",c.area());
}
