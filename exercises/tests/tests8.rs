// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.


fn main() {}
//cfg 是 Rust 中的 条件编译（conditional compilation） 机制的一部分
//它允许根据编译时的配置决定哪些代码片段应该被包含或排除
//在 Windows 上编译，#[cfg(target_os = "windows")] 对应的代码会被编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
