use std::f64::consts; // std.f64.consts

// local .rs dosyaları
mod include_me;
mod command_line;
mod structs;

fn main() {

    // değişken default olarak immutable
    // mut keyword u ile mutable oluyor
    let mut sum = 0;
    for i in 1..5 {
        sum += i;
    }
    println!("{}", sum);


    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2); // 11 42

    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res); // 1

    // let bigint: i64 = 0; type after variable
    let x = 2.0 * consts::PI; // neden olum?
    let abs_difference = (x.cos() - 1.0).abs();
    assert!(abs_difference < 1e-10);

    let arr = [10, 11, 12, 13];
    println!("{:?}", arr);
    println!("################");
    call_another_function();

}

fn call_another_function() {
    let res = square(2.0);
    // println!("{}", res); // 4

    let res = factorial(2);
    // println!("{}", res); // 4

    // mathematical();
    call_function_from_another_file();
}

fn square(x: f64) -> f64 {
    // return keyword u kullanılmayabilir
    // return x * x
    x * x
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

//  & ref yaratır
fn by_ref(x: &i32) -> i32{
    *x + 1
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn mathematical() {
    let pi: f64 = 3.1416;
    let x = pi/2.0;
    let cosine = x.cos();
    println!("{}", cosine);
}

fn call_function_from_another_file() {

    // include_me::array();

    let arr = [10,20,30,40];
    let res = include_me::sum(&arr);     // look at that &
    // println!("sum {}", res);

    // include_me::array_types();
    // include_me::slice();
    // include_me::vectors();
    // include_me::iterator();
    // include_me::more_about_vectors();
    // include_me::strings();
    // include_me::multilingual();
    // command_line::command_line();
    structs::test_struct();
}