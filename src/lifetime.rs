// All references are borrowed from some value, and all values have lifetimes.
// The lifetime of a reference cannot be longer than the lifetime of that value.
// Rust cannot allow a situation where that reference could suddenly become invalid.
#[derive(Debug)]
struct A {
    s: &'static str
}

#[derive(Debug)]
struct B <'b> {
    s: &'b str
}

pub fn test_lifetime() {
    let a = A { s: "hello dammit" };
    println!("{:?}", a);

    let s = "I am a little string".to_string(); // After this point, our a struct and the s string are bound by a strict contract:
    let a = B { s: &s }; // a borrows from s, and cannot outlive it.
    println!("{:?}", a);
}