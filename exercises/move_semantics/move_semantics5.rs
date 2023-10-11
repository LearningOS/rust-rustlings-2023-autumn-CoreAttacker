// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() { //在同一个作用域内，对同一个变量 x 创建了两个可变引用 y 和 z。这违反了 Rust 的借用规则，
    //因为 Rust 不允许同一个作用域内存在多个可变引用指向同一个值，以防止数据竞争和内存不安全。
    //普通引用是可以同时存在多个的
    let mut x = 100;
    {
        let y = &mut x;
        *y += 100;
    }
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
