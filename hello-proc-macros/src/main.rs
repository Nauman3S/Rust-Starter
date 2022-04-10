
use hello_proc_macro_lib::HelloMacro;
use hello_proc_macro_lib_derive::HelloMacro;

#[derive(HelloMacro)]
pub struct PankCakes;

fn main() {
    PankCakes::hello_macro();
}
