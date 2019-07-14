extern crate bcrypt;
use std::fmt;


use bcrypt::{DEFAULT_COST, hash, verify};


#[allow(dead_code)]
enum Errors {
    NameError,
    AgeError,
    EmailError
}

#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    email: String,
    password: String
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.first_name, self.last_name, self.email, self.password)
    }
}

impl User {

    fn new(first: &str, last: &str, email: &str, pass: &str) -> User {
        User {
            first_name: first.to_string(),
            last_name: last.to_string(),
            email: email.to_string(),
            password: pass.to_string()
        }
    }

    fn user_info(&self) -> String {
        format!("{} {} {}", self.full_name(), self.email, self.password)
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name.to_ascii_uppercase(), self.last_name.to_ascii_uppercase())
    }

    fn change_email(&mut self, email: &str) {
        self.email = email.to_string();
    }

    fn password(&self) {
        println!("{}", self.password);
    }

    fn password_encoder(&mut self) {
        match hash(&self.password, DEFAULT_COST) {
            Ok(n) => self.password = n,
            Err(err) => println!("Error: {}", err),
        }
    }

    fn login(&mut self, pass: &str) {       
        match verify(pass, &self.password) {
            Ok(n) => if n == true {println!("login succesfull")} else {println!("password doesnt match")},
            Err(err) => println!("Error: {}", err),
        }
    }
}

pub fn test() {
    let mut user = User::new("avni", "genc", "avnignc@gmail.com", "123");
    println!("{:?}", user);
    println!("{}", user.full_name());
    println!("{}", user.user_info());
    user.change_email("newemail@gmail.com");
    println!("{}", user.user_info());
    user.password();
    user.password_encoder();
    user.password();
    println!("{}", user.user_info());
    user.login("1234");
}