fn main() {
    let mut s = String::from("Hello world");
    s = s + " 00 ";
    s.push_str(" 1123 ");
    println!("{}", s);

    let mut s2 = String::new();
    s2 = s.clone(); //becsue s2=s only copies pointer and invalidates the s; clone is used for copying/deep copy

    let (s3, len) = calculate_length(s2);

    println!("The length of '{}' is {}.", s3, len);

    println!("s {} and s2 {}", s, s3);

    take_ownership(s);
    //println!("old s {}",s);// will not work as s is moved out of the scope

    let in_val = 321;
    println!("in_val {}", in_val);
    make_copy(in_val);
    println!("in_val {}", in_val); //works becasue int, bool, float, char implements 'Copy'
}

fn take_ownership(str: String) {
    println!("moved string {}", str);
}

fn make_copy(intt: i32) {
    println!("i32: {}", intt);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}