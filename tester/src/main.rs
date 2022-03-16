use std::io;

fn main() {
    println!("Hello, world!");

    let mut a = 32;
    a = a + 1;
    println!("the number is {}", a);

    let mut b: u128 = 12312;
    b = b + 1;
    println!("data type{}", b);

    let arr = [1; 5];
    for number in arr {
        println!("Simple For {}", number);
    }

    for number in 1..4 {
        println!("Numbered {}", arr[number]);
    }

    println!("area of sq of l=3 is {}", get_area_square(3));

    println!("something strange:  {}", {
        let x = 33;
        x
    });

    let msg= String::from("str value");
    println!("string {}",msg);


    let mut num=String::new();

    io::stdin().read_line(&mut num).expect("failed");
    println!("read the num: {}",num);
}

fn get_area_square(n: u32) -> u32 {
    let mut ng: u32 = n;
    ng = ng * 2;
    ng
}
