#[derive(Debug)]
struct Student {
    no: i32,
    name: String,
    surname: String,
    class: Class,
} 

#[derive(Debug)]
struct Class {
    no: i32,
    name: String,
}

impl Student {
    fn new(no: i32, name: &str, surname: &str, class: &str) -> Student {
        Student {
            no: no,
            name: name.to_string(),
            surname: surname.to_string(),
            class: Class::new(no, class),
        }
    }

    fn get_student_information(&self) -> String {
        format!("NO: {} {} {}", self.no, self.name, self.surname)
    }
}

impl Class {
    fn new(no: i32, name: &str) -> Class {
        Class {
            no: no,
            name: name.to_string()
        }
    }
}
// #[allow(dead_code)] -> kullanılmayan fonksiyonların başına konulduğunda compile aşamasında warning vermesini engelliyor.
// kullanılmayan değişkenlerin önüne  _ (underscore) konulursa yine aynı şekilde warning almıyoruz. 
#[allow(dead_code)]
fn unused_function() {
    println!("unused output");
}

#[allow(dead_code)]
pub fn test() {
    let student1 = Student::new(1, "Avni", "Genç", "1.sınıf");
    println!("{:?}", student1);
    println!("No: {} - Ad: {}, Soyad: {}", student1.no, student1.name, student1.surname);
    println!("Info: {}", student1.get_student_information());

}