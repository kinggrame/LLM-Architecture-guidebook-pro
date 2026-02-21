fn main() {
    // Demo 1: 基本 trait
    trait_basic_demo();

    // Demo 2: Trait object
    trait_object_demo();

    // Demo 3: 标准库 trait
    standard_traits_demo();
}

fn trait_basic_demo() {
    println!("=== Demo 1: 基本 Trait ===");

    trait Printable {
        fn print(&self);
    }

    struct Point {
        x: i32,
        y: i32,
    }
    struct Person {
        name: String,
        age: u32,
    }

    impl Printable for Point {
        fn print(&self) {
            println!("Point({}, {})", self.x, self.y);
        }
    }

    impl Printable for Person {
        fn print(&self) {
            println!("Person({}, {})", self.name, self.age);
        }
    }

    let p = Point { x: 10, y: 20 };
    p.print();

    // trait 作为函数参数
    fn print_it(p: &impl Printable) {
        p.print();
    }
    print_it(&p);
}

fn trait_object_demo() {
    println!("\n=== Demo 2: Trait Object ===");

    trait Drawable {
        fn draw(&self);
    }

    struct Circle {
        radius: f64,
    }
    struct Rect {
        w: f64,
        h: f64,
    }

    impl Drawable for Circle {
        fn draw(&self) {
            println!("Circle r={}", self.radius);
        }
    }

    impl Drawable for Rect {
        fn draw(&self) {
            println!("Rect {}x{}", self.w, self.h);
        }
    }

    // 动态分发
    let shapes: Vec<&dyn Drawable> = vec![&Circle { radius: 5.0 }, &Rect { w: 10.0, h: 20.0 }];
    for s in shapes {
        s.draw();
    }
}

fn standard_traits_demo() {
    println!("\n=== Demo 3: 标准库 Trait ===");

    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Default for Point {
        fn default() -> Self {
            Point { x: 0, y: 0 }
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p1 = Point::default();
    let p2 = p1;
    println!("Default: {}", p1);
    println!("Copy: p1={}, p2={}", p1.x, p2.x);
}
