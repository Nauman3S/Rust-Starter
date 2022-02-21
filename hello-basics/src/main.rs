fn main() {
    println!("Hello, Basics!");
    println!("{}",printer(32));//test functions
}


fn printer(x: i32)->i32{
    println!("printer");
    let sp="9988";
    let intt: i32=sp.parse().expect("nan");

    println!("str len{}  intt{}  arg{}",sp.len(),intt,x);

    let dx=3;

    dx//return val
}
