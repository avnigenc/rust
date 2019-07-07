
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
    println!("last -> {:?}", last); // -1

}

/*
 In systems languages, program memory comes in two kinds: the stack and the heap.
 It is very fast to allocate data on the stack, but the stack is limited; typically of the order of megabytes.
 The heap can be gigabytes, but allocating is relatively expensive, and such memory must be freed later.
 When a vector is modified or created, it allocates from the heap and becomes the owner of that memory.
 The slice borrows the memory from the vector. When the vector dies or drops, it lets the memory go.
*/
pub(crate) fn vectors() {
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