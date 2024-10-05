// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.



//macro_rules! 是 Rust 语言中定义宏的一种方式
//宏模块需要声明
#[macro_use]
pub mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    
    my_macro!();
}
