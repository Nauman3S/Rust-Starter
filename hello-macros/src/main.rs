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

macro_rules! add_it {
    (plz $l:expr; addInto $r:expr; then;show_res;~) => {
        println!("adding l and r {:?}  =  {:?}", stringify!($l + $r), $l + $r)
    };
}

fn main() {


    let v1: Vec<u32>=vec!{1,2,87};//for macros bracket types don't matter
    println!("v1[3] = {}",v1[2]);


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

    add_it!(    plz 3;
                addInto 2;
                then;
                show_res;
                    ~);
}
