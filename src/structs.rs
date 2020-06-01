#![allow(dead_code)]

use std::fmt;

#[derive(Debug)] // makes this printable by `fmt::Debug`
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub name: String,
}

impl Vertex {
    pub fn clone(&self) -> Vertex {
        Vertex {
            x: self.x,
            y: self.y,
            // The clone is needed when the value we're cloning is on the heap. Values on the stack are always cloned.
            name: self.name.clone(),
        }
    }
}

// A destructor.
// This needs to be in a separate section in order to be triggered on `drop`
impl Drop for Vertex {
    fn drop(&mut self) {
        println!("Dropping a Vertex!");
    }
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: String::from(first),
            last_name: String::from(last),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn name_to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

#[derive(Debug)]
struct Lifetime<'a> {
    field: &'a str,
}

impl<'a> Lifetime<'a> {
    pub fn get_field(&self) -> &'a str {
        self.field
    }
}

impl<'b> fmt::Display for Lifetime<'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.field)
    }
}

fn consume(_s: String) {}

pub fn run() {
    let mut p = Person::new("John", "Doe");
    p.set_last_name("Gray");
    println!("{:?}", p.name_to_tuple());

    let f = String::from("field");

    let l = Lifetime { field: &f };
    println!("{} {}", l.get_field(), l);
}
