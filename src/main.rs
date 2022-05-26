#![allow(dead_code)]

// use hello_lib;

use std::fmt::{Display, Formatter};

mod guess;
mod structs;

struct BagOfHolding<T> {
    item: Option<T>,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct Foo<'a> {
    i: &'a Point,
}

/// Consumes a variable, making it unavailable in the code that follows.
fn consume<T: Display>(a: T) {
    match a {
        _ => println!("consumed {}!", a),
    }
}

/// Returns "hello" <- docs for the following item (mind the triple slash).
fn hello() -> &'static str {
    //! Also returns a static string. <- docs for the containing item (mind the bang).
    let greeting = "hello";
    println!("{}", greeting);
    &greeting
}

#[allow(unused_macros)]
macro_rules! hi {
    () => {
        hello();
    };
}

fn output() {
    const PI: f64 = std::f64::consts::PI;
    println!("Pi is roughly {:.2} or {}", PI, PI);

    // multi-use params, positional params, named params:
    println!(
        "{0}, this is {b}. {b}, this is {0}. Emoji: {emoji}",
        "Alice",
        b = "Bob",
        emoji = '\u{1F600}'
    );
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 2600, 2600, 2600);
    println!("Tuple: {:?}", (3.14, "Mark"));
}

fn tuples() {
    let mut tuple = ("a", 22.0 / 7.0, 74);
    tuple.0 = hello();
    let (_, pi, _) = tuple;
    println!("{:?}, {}", tuple.0, pi);
}

fn vars_loops_arrays_slices() {
    for i in 0..=10 {
        // inclusive range, 0..10 is exclusive
        print!("{} ", i);
    }
    println!();

    let (my_name, mut my_age) = ("Jack", 25); // multi-assign
    my_age += 1;

    let person: (&str, &str, u8) = (my_name, "Wyoming", my_age); // tuples
    println!("{} lives in {} and is {}", person.0, person.1, person.2);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr is {:?} bytes", std::mem::size_of_val(&arr));

    let slice: &[i32] = &arr[..2]; // [1..3], [1..]
    println!("slice: {:?}", slice);

    let mut v = vec![1, 2, 3, 4]; // optionally, you can let v: Vec<i32> = ...
    v.push(7);
    println!("vector: {:?}", v);

    for i in v.iter_mut() {
        *i = *i * 2;
        print!("{:?} ", i);
    }
    println!();
}

fn arrays() -> [u8; 8] {
    // let mut arr: [u8; 3]; // uninitialised
    let mut array = [0u8; 8]; // auto-initialised with 0s. notice the `;`
    array[1] = 5;
    println!("{:?}, {}", array, array.len());
    return array;
}

fn loops() {
    let mut counter = 0;
    let _loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    for _ in arrays().iter() {}
    // using a Range
    for x in (1..4).rev() {
        println!("{} ", x);
    }
}

fn copy_clone() {
    // copy and clone
    #[derive(Copy, Clone, Debug)]
    pub struct Vertex {
        pub x: f64,
        pub y: f64,
    }

    let v = Vertex { x: 1.0, y: 2.0 };
    let w = v.clone();
    println!("{:?}", w);
    let s = String::from("foo");
    println!("{}", s);
}

fn ownership() {
    fn take_and_give(some_str: String) -> String {
        some_str
    }

    let mut s = String::from("foo");
    // s goes out of scope with the call to take_and_give
    // and comes back in scope with the return from it
    s = take_and_give(s);
    println!("{}", s);
}

fn borrowing() {
    // borrow borrows s, so its value is not moved and it's still available
    fn borrow(some_str: &String) -> usize {
        some_str.len()
    }

    let mut s = String::from("foo");
    let mut l = borrow(&s);
    println!("the length of {} is {}", s, l);

    // borrow_mut borrows s as mutable in order to modify it but since it's not
    // moved it's still available after the call
    fn borrow_mut(some_str: &mut String) -> usize {
        some_str.push_str("bar"); // modify the string
        some_str.len()
    }
    l = borrow_mut(&mut s);
    println!("the length of {} is {}", s, l);

    // NOTE
    // Note that a reference’s scope starts from where it is introduced and
    // continues through the last time that reference is used. For instance,
    // this code will compile because the last usage of the immutable references
    // occurs before the mutable reference is introduced:
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn slices() {
    let a = [0i32; 8];
    let s = &a[1..5]; // mind the ampersand
    println!("{:?}", s);
}

fn structs() {
    #[derive(Debug)] // makes this printable by `fmt::Debug`: println!("{:?}", r)
    pub struct Rect {
        a: u32,
        b: u32,
        name: String,
    }

    impl Rect {
        pub fn area(&self) -> u32 {
            self.a * self.b
        }

        pub fn clone(&self) -> Rect {
            Rect {
                a: self.a,
                b: self.b,
                name: self.name.clone(), // clone is needed when the variable is on the heap
            }
        }
    }

    // A destructor. This needs to be in a separate section in order to be triggered on `drop`
    impl Drop for Rect {
        fn drop(&mut self) {
            println!("Dropping Rect {}!", self.name);
        }
    }

    let r = Rect {
        a: 14,
        b: 23,
        name: String::from("yolo"),
    };
    println!(
        "The area of a rectangle with sides {} and {} is {}",
        r.a,
        r.b,
        r.area()
    );
    let q = r.clone();
    println!("q is {:?}, p is {:?}", q, r);
}

fn matching() {
    let n = 42;

    match n {
        x @ 42 => println!("expect 42: {}", x),
        _ => {
            println!("not x");
        }
    }
}

fn main() {
    // tuples();
    // arrays();
    // loops();
    // copy_clone();
    // ownership();
    // borrowing();
    // slices();
    // println!("{}", hello_lib::gcd(165, 35)); // TODO How to publish a lib as a crate?
    // structs::run();
    // structs(); // TODO Continue here in the Book of Rust.
    // matching();

    scratch().unwrap();
}

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(-13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}
fn foo(x: Point) {
    println!("{}", x);
}

fn scratch() -> Result<(), String> {
    let p = Point { x: 1, y: 2 };

    foo(p);

    println!("{}", p);
    Ok(())
}
