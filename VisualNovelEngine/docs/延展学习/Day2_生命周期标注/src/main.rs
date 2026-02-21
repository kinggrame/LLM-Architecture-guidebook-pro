fn main() {
    // Demo 1: 生命周期基本使用
    println!("=== Demo 1: 生命周期基本使用 ===");

    let result = longest("hello", "world!");
    println!("最长的: {}", result);

    // Demo 2: 结构体中的生命周期
    println!("\n=== Demo 2: 结构体中的生命周期 ===");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt::new(&novel);
    println!("引用: {}", excerpt.part);

    // Demo 3: 方法中的生命周期
    println!("\n=== Demo 3: 方法中的生命周期 ===");

    let result = excerpt.announce_and_return("重要公告");
    println!("返回: {}", result);

    // Demo 4: 静态生命周期
    println!("\n=== Demo 4: 静态生命周期 ===");

    let static_str: &'static str = "我永远不会过期";
    println!("静态: {}", static_str);

    // Demo 5: 多个生命周期
    println!("\n=== Demo 5: 多个生命周期 ===");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt { part: first };
    let s = "hello";
    let r = excerpt.announce_and_return(s);
    println!("结果: {}", r);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn new(text: &'a str) -> Self {
        ImportantExcerpt { part: text }
    }

    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("公告: {}", announcement);
        self.part
    }
}
