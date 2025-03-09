mod my_module;

use my_module::my_function;
use my_module::outer::inner::inner_function;

fn main() {
    my_function();
    inner_function();
}