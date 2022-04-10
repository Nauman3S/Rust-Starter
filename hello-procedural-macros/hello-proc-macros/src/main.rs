
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
pub struct Pankcakes;

fn main() {
    Pankcakes::hello_macro();
}
