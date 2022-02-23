fn main() {
    println!("Hello, Basics!");
    println!("{}", printer(32)); //test functions

    let mut n: u32 = 1; //unsigned 32bit int
    n = n + 1;
    let mut float_y: f32 = 1.1; //32bit float
    float_y = float_y + 3.2;
    println!("int u32 {}", n);
    println!("float 32 {}", float_y);

    let tup: (i32, f32, i32) = (200, 2.2, 3); //tuple
    println!("tuple + float {}", tup.1 + float_y);

    let (_x, y, _z) = tup;
    println!("tuple + float {}", y + float_y);

    let array = [1, 2, 98];

    println!("array {}", array[2]);

    let array_prefilled = [5; 10];

    println!("array prefilled {}", array_prefilled[1]);

    let array_predefined: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array predefined {}", array_predefined[3]);

    // stackOverFlow(1);//will overflow the stack by calling iteself recuresively
}
fn stackOverFlow(x: u32) -> u32 {
    println!("{}", x);
    stackOverFlow(x + 1)
}
fn printer(x: i32) -> i32 {
    println!("printer");
    let sp = "9988";
    let intt: i32 = sp.parse().expect("nan");

    println!("str len{}  intt{}  arg{}", sp.len(), intt, x);

    let dx = 3;

    dx //return val
}
