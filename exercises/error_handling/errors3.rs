// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;


//Rust 程序的标准返回类型:(),表示“无返回值”

fn main() ->Result<(), ParseIntError> {
//因为total_cost有概率返回Err，若返回Err，Err类型的e不能和token比较,main函数也该直接返回Err
//? 运算符用于错误传播：如果 total_cost 返回 Ok,则 ? 运算符会提取 Ok 中的值并继续执行；
//如果 total_cost 返回 Err，则 main 函数会立即返回这个 Err 值，并且不再继续执行后续代码
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
