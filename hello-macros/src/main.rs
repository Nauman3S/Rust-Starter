macro_rules! a_macro {
    () => {
        println!("hello macro!")
    };
}

macro_rules! x_and_y {
    (x=> $e:expr) => {
        println!("X: {}", $e)
    };
    (y=>$e:expr) => {
        println!("Y: {}", $e)
    };
}

macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! print_expression {
    ($e:expr) => {
        println!("{:?} = {:?}", stringify!($e), $e)
    };
}

fn main() {
    a_macro!();
    x_and_y!(x=>19);
    x_and_y!(y=>29+2);

    build_fn!(HiThere);
    HiThere();
    print_expression!({
        let x = 3;
        let mut y = 1;
        y += 1;
        x + y
    });
}