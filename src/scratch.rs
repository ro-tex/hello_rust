#![allow(dead_code)]
#![allow(unused_imports)]

use std::any::Any;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::ops::Drop;
use std::rc::Rc;

#[allow(unused)] // I know this is unused, don't warn me about it.
macro_rules! swap {
    (&mut v: Vec!(i32), i, j: usize) => {
        println!("swap indices {:?} and {:?}", i, j);
        println!(" >>> before {:?}", v);
        (v[i], v[j]) = (v[j], v[i]);
        println!(" >>> after  {:?}", v);
    };
}

trait Greeter {
    fn greet(&self);
}

trait Namer {
    fn name(&self) -> String;
}

#[derive(Debug, Default)]
struct User {
    name: String,
}

impl User {
    // hey is a static method.
    fn hey() -> String {
        "Hey!".to_string()
    }
}

impl Greeter for User {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

impl Namer for User {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Drop for User {
    fn drop(&mut self) {
        if self.name.is_empty() {
            self.name = "<nameless>".to_string();
        }
        println!(">>> Dropping {}", self.name)
    }
}

// fn foo<T: Greeter>(thing: &T) { // same as below
fn foo(thing: &impl Greeter) {
    thing.greet();
}

fn bar<T>(thing: &T)
where
    T: Greeter,
    T: Namer,
{
    println!("Name: {}", thing.name());
}

fn unsafe_play_with_bytes() {
    let mut n: u16 = (1 << 10) + (1 << 2); // 1024 + 4
    println!("n  is {:?}", n);
    let p1 = &mut n as *mut u16 as usize;
    println!("p1 is {:?}", p1);
    let p2 = p1 + 1; // point to the second byte, i.e. 4
    println!("p2 is {:?}", p2);
    let fb: u8 = unsafe { *(p1 as *mut u8) }; // grab the value of the first byte into a var
    println!("fb is {}", fb);
    let mut sb: u8 = unsafe { *(p2 as *mut u8) }; // grab the value of the second byte into a var
    println!("sb is {}", sb);
    // check that the 4 is stored in new memory
    println!("&sb is {:?}", (&mut sb as *mut u8 as usize));
    // increment the second byte with 2, i.e. 1<<1. since this is little endian,
    // this will result in n being incremented with 1<<(1+8), i.e. 512.
    unsafe { *(p2 as *mut u8) += 2 };
    println!("n  is {:?}", n);
}

fn unsafe_alloc() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);

        *(ptr as *mut u16) = 1042;
        // read the least significant byte and make sure it's 1042-1024=18
        assert_eq!(*(ptr as *mut u8), 18);
        println!("{}", *(ptr as *const u16));

        dealloc(ptr, layout);
    }
}

fn consume<T: std::fmt::Debug>(p: T) {
    println!(
        "consuming address {:?}, value {:?}",
        &p as *const T as usize, p
    );
}

fn fallible() -> Result<String, String> {
    Err("a fallible error".to_string())
}

fn memory_play() {
    let mut u = User {
        name: "stack user one".to_string(),
    };
    println!("stacked: {} {}", u.name, &u as *const User as usize);
    u = User {
        name: "stack user two".to_string(),
    };
    println!("stacked: {} {}", u.name, &u as *const User as usize);

    let u = Box::new(User {
        name: "box user".to_string(),
    });
    println!(
        "boxed:   {}       {}",
        u.name, &u as *const Box<User> as usize
    );

    let mut rcu = Rc::new(User {
        name: "rc user one".to_string(),
    });
    println!("rc -> {:?}", &rcu as *const Rc<User> as usize);
    rcu = Rc::new(User {
        name: "rc user two".to_string(),
    });
    println!("rc -> {:?}", &rcu as *const Rc<User> as usize);
    let u2 = rcu.clone();
    println!("rc -> {:?}", &u2 as *const Rc<User> as usize);
    let u3 = rcu.clone();
    println!("rc -> {:?}", &u3 as *const Rc<User> as usize);
    consume(rcu);
    consume(u2);
    consume(u3);
}

#[derive(Debug)]
struct MyError;
impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "err!")
    }
}
impl Error for MyError {}

pub fn main() {
    match fallible() {
        Ok(_) => println!("ok"),
        Err(e) => println!("err: {}", e),
    }
}
