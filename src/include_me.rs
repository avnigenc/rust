
pub(crate) fn array() {
    let array = [10, 11, 12, 13];

    println!("first -> {}", array[0]);

    for i in 0..array.len() {
        println!("[{}] -> {}", i, array[i]);
    }

    println!("len -> {}", array.len());



}

pub(crate) fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

pub(crate) fn slice() {
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


}


pub(crate) fn array_types() {
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