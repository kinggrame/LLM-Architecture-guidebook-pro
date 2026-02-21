fn main() {
    // Demo 1: Box - 堆分配
    println!("=== Demo 1: Box ===");
    let b = Box::new(5);
    println!("Box value: {}", b);

    // Demo 2: Rc - 引用计数
    println!("\n=== Demo 2: Rc ===");
    use std::rc::Rc;
    let data = Rc::new(vec![1, 2, 3]);
    let clone1 = Rc::clone(&data);
    let clone2 = Rc::clone(&data);
    println!("引用计数: {}", Rc::strong_count(&data));

    // Demo 3: RefCell 和 interior mutability
    println!("\n=== Demo 3: RefCell ===");
    use std::cell::RefCell;
    let x = RefCell::new(42);
    *x.borrow_mut() = 100;
    println!("RefCell value: {}", x.borrow());

    // Demo 4: Arc - 线程安全
    println!("\n=== Demo 4: Arc ===");
    use std::sync::{Arc, Mutex};
    use std::thread;

    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Arc result: {}", *data.lock().unwrap());
}
