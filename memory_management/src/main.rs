use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("🧠 Memory Management Demo in Rust");

    let s1 = String::from("Ownership Example");
    let s2 = s1; // s1 is moved to s2 
    println!("Ownership transferred: {}", s2);

    // borrowing immutable
    let s3 = String::from("Borrowing Example ");
    borrow_demo(&s3);
    println!("✅ After borrow: {}", s3);

    // mutable borrowing

    let mut s4 = String::from("Hello");
    mutate_demo(&mut s4);
    println!("After mutation: {}", s4);

    // lifetimes

    let result;
    let a = String::from("abcd");
    {
        let b = String::from("xyzttt");
        result = longest(&a, &b); // longest returns the longest of two string slices.
        println!("⏳ Longest string: {}", result);
    }

    // heap allocation

    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed);

    // Rc (Reference Counted Smart Pointer)
    //Rc stands for Reference Counted.   Allows multiple ownership of data.   Useful in scenarios like shared nodes in a tree.

    let rc_value = Rc::new(String::from("shared"));
    let rc_clone = Rc::clone(&rc_value);

    println!("📚 Rc values: {}, {}", rc_value, rc_clone);
    println!("Ref count: {}", Rc::strong_count(&rc_value));

    //   RefCell (Interior Mutability)

    let cell = RefCell::new(200); // RefCell allows mutable access to data even when it's not declared as mutable.
    *cell.borrow_mut() += 50; // Use .borrow_mut() to mutate, .borrow() to read.

    println!("🧪 RefCell value: {}", cell.borrow());
}

fn borrow_demo(data: &String) {
    // this simple function is to show that we can borrow without taking ownership
    println!("📥 Borrowed: {}", data);
}

fn mutate_demo(data: &mut String) {
    //  This shows how to change something by passing a mutable reference.
    data.push_str("world");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // here we are saying  the returned reference lives as long as the shorter-lived input.
    if x.len() > y.len() { x } else { y }
}
