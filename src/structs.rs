#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

// no self argument: you can associate functions with structs, like the new "constructor".
// &self argument: can use the values of the struct, but not change them
// &mut self argument: can modify the values
// self argument: will consume the value, which will move.
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String { // ( &self ) as short for ( self: &Person )
        format!("{} {}", self.first_name, self.last_name)
    }
    
    #[allow(dead_code)]
    fn copy(&self) -> Self {
        Self::new(&self.first_name, &self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

#[allow(dead_code)]
pub fn test_struct() {
    let p = Person {
        first_name: "Avni".to_string(),
        last_name: "Genç".to_string()
    };

    println!("{} {}", p.first_name, p.last_name);

    let p1 = Person::new("Avni", "Genç");
    println!("Ad: {}, Soyad: {}", p1.first_name, p1.last_name);
    println!("Ad-Soyad: {}", p1.full_name());

    let mut p2 = Person::new("Ali", "Veli");
    println!("{:?}", p2); // Person { first_name: "Ali", last_name: "Veli" }
    p2.set_first_name("Veli");
    println!("{:?}", p2); // Person { first_name: "Veli", last_name: "Veli" }
    println!("{:?}", p2.to_tuple()); // ("Veli", "Veli")


}