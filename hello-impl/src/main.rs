struct Rectangle{
    width: i32,
    height: i32,
}

impl Rectangle{

    fn area(&self)-> i32{
        self.width*self.height
    }
}

fn main() {
    let rect1= Rectangle{
        width:12,
        height:32,
    };
    println!("area of an implemented rect is {}",rect1.area());
}
