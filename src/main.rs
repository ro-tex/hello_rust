#![allow(dead_code)]

fn hello() -> &'static str {
    &"hello"
}

fn tuples() {
    let mut tuple = ("a", 22.0 / 7.0, 74);
    tuple.0 = hello();
    let (_, pi, _) = tuple;
    println!("{:?}, {}", tuple.0, pi);
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
    String::from("foo");
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
    // this is a struct
}

fn main() {
    tuples();
    arrays();
    loops();
    copy_clone();
    ownership();
    borrowing();
    slices();
    structs();
}
