// Demo 1: 基本所有权演示
// 运行: cargo run --bin ownership_basics

fn main() {
    println!("=== 演示1: 所有权转移 ===");

    // s1 拥有 String 的所有权
    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    // 所有权从 s1 转移到 s2
    let s2 = s1;
    // println!("s1 = {}", s1); // 编译错误！s1 不再有效
    println!("s2 = {}", s2);

    println!("\n=== 演示2: 作用域结束自动释放 ===");

    {
        let s3 = String::from("world");
        println!("s3 = {}", s3);
    } // s3 在这里被自动释放（drop）

    println!("s3 已经不存在了");

    println!("\n=== 演示3: 函数参数的所有权 ===");

    let s4 = String::from("ownership");
    takes_ownership(s4);
    // println!("s4 = {}", s4); // 编译错误！s4 的所有权已转移

    println!("\n=== 演示4: 返回值的所有权 ===");

    let s5 = String::from("返回值");
    let s6 = gives_ownership();
    println!("s5 = {}", s5);
    println!("s6 = {}", s6);

    // ===== Demo 2: 借用 =====
    borrowing_demo();

    // ===== Demo 3: 借用检查器实战 =====
    borrow_checker_demo();
}

fn takes_ownership(s: String) {
    println!("收到所有权: {}", s);
}

fn gives_ownership() -> String {
    String::from("新创建的值")
}

fn borrowing_demo() {
    println!("\n=== 演示1: 不可变借用 ===");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("s1 = {}, 长度 = {}", s1, len);

    println!("\n=== 演示2: 可变借用 ===");

    let mut s2 = String::from("hello");
    modify_string(&mut s2);
    println!("修改后: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify_string(s: &mut String) {
    s.push_str(", world");
}

fn borrow_checker_demo() {
    println!("\n=== 借用规则演示 ===");

    let mut s = String::from("hello");

    let r1 = &s;
    println!("r1 = {}", r1);

    // r1 的作用域结束后，才可以创建可变借用
    let r2 = &mut s;
    r2.push_str(" world");
    println!("r2 = {}", r2);
}
