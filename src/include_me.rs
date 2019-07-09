
pub fn array() {
    let array = [10, 11, 12, 13];

    println!("first -> {}", array[0]);

    for i in 0..array.len() {
        println!("[{}] -> {}", i, array[i]);
    }

    println!("len -> {}", array.len());



}

pub fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

pub fn slice() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);

    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last); // none

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap()); // panic

// Note the * - the precise type inside the Some is &i32, which is a reference.
// We need to dereference this to get back to a i32 value.
//
//   let maybe_last = slice.get(5);
//    let last = if maybe_last.is_some() {
//        *maybe_last.unwrap()
//    } else {
//        -1
//    };

    // üstteki kodun kısa hali
    // eğer get(index) var ise döndür yoksa unwrap_or(& value) döndür
    // &
    let last = *slice.get(5).unwrap_or(&-1);
    println!("last -> {:?}", last); // -1

}

/*
 In systems languages, program memory comes in two kinds: the stack and the heap.
 It is very fast to allocate data on the stack, but the stack is limited; typically of the order of megabytes.
 The heap can be gigabytes, but allocating is relatively expensive, and such memory must be freed later.
 When a vector is modified or created, it allocates from the heap and becomes the owner of that memory.
 The slice borrows the memory from the vector. When the vector dies or drops, it lets the memory go.
*/
pub fn vectors() {
    // mutable olmalı
    let mut vector = Vec::new();
    vector.push(10);
    vector.push(20);
    vector.push(30);

    println!("vector {:?}", vector); // [10, 11, 12, 13]


    let first = vector[0];  // will panic if out-of-range
    let maybe_first = vector.get(0);

    println!("first is {}", first); // first is 10
    println!("maybe_first is {:?}", maybe_first); // maybe_first is Some(10)
}

pub fn strings() {

    // Create an empty and growable `String`
    let mut string = String::new();

    // Heap allocate a string
    let alice = String::from("I like dogs");

    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    let text = "hello dolly";  // the string slice
    let s = text.to_string();  // it's now an allocated string

    dump(text);
    dump(&s);

    if text.contains("e") {
        println!("{} contains {}", text, "e");
    }


    let mut s = String::new();
    s.push('H');
    s.push_str("ello");
    s += " World!"; // short for `push_str`
    println!("{}", s); // Hello World!

    s.pop();
    println!("{}", s); // Hello World
    assert_eq!(s, "Hello World");

    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);
    println!("{}", res);

    assert_eq!(res, "hello [10,20,30]");

    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];
    println!("slices {:?} {:?}", text_s, string_s); // slices "tatic" "na"

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram); // Pangram: the quick brown fox jumps over the lazy dog

    let mut vec = vec![];
    for word in pangram.split_whitespace().rev() { // rev() -> reverse
        vec.push(word);
    }
    println!("{:?}", vec); // ["dog", "lazy", "the", "over", "jumps", "fox", "brown", "quick", "the"]

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("{:?}", chars); // [' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']


    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }
    println!("{}", string); //  , a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str); // Used characters: a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z

    let text = "the red fox and the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("{:?}", words); // ["the", "red", "fox", "and", "the", "lazy", "dog"]

    let mut words = Vec::new();
    words.extend(text.split_whitespace());
    println!("{:?}", words); // ["the", "red", "fox", "and", "the", "lazy", "dog"]

    let stripped: String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect();
    println!("{}", stripped); // theredfoxandthelazydog
}

pub fn iterator() {
    let mut iter = 0..3; // like range in python
    println!("iter -> {:?}", iter.next()); // Some(0)
    println!("iter -> {:?}", iter.next()); // Some(1)
    println!("iter -> {:?}", iter.next()); // Some(2)
    println!("iter -> {:?}", iter.next()); // None

    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }

    let sum: i32  = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);

    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s); // [1, 2] [2, 3] [3, 4] [4, 5]
    }

    for s in slice.chunks(2) {
        println!("chunks {:?}", s); // [1, 2] [3, 4] [5]
    }
}


pub fn more_about_vectors() {
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);
    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);


    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    println!("befor sort -> {:?}", v1);
    v1.sort();
    println!("after sort -> {:?}", v1);
    v1.dedup(); // remove duplicated elements
    println!("after dedup -> {:?}", v1);
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}

pub fn array_types() {
    // only one type
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
}

pub fn dump(s: &str) {
    println!("str '{}'", s);
}

// But, you cannot index strings!
// This is because they use the One True Encoding, UTF-8, where a 'character' may be a number of bytes.
pub fn multilingual() {
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch); // 'H' 'i' '!' ' ' '¡' 'H' 'o' 'l' 'a' '!' ' ' 'п' 'р' 'и' 'в' 'е' 'т' '!'
    }
    println!("");
    println!("len {}", multilingual.len()); // len 25 (bytes)
    println!("count {}", multilingual.chars().count()); // count 18

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}