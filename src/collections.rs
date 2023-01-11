use std::collections::HashMap;

pub fn _common_collections() {
    _cc_vec();
    _cc_string();
    _cc_hashmap();
}

/** hash maps store their data on the heap.
keys are unique but values aren't.
*/
fn _cc_hashmap() {
    let c1 = String::from("India");
    let c2 = String::from("Australia");

    let mut hm1 = HashMap::new();
    hm1.insert(&c1, 285);
    /// Directly override the value
    hm1.insert(&c1, 288);
    hm1.insert(&c2, 321);
    println!("Hashmap1 = {:?}", hm1);
    hm1.remove(&c1);
    println!("Hashmap1 = {:?}", hm1);

    let mut hm2 = HashMap::new();
    /// String c1 and c2 moved to HashMap. Hence, its value can't be borrowed later.
    hm2.insert(c1, 200);
    hm2.insert(c2, 300);
    println!("Hashmap2 = {:?}", hm2);
    hm2.remove("Australia");
    println!("Hashmap2 = {:?}", hm2);

    let mut hm3 = HashMap::new();
    hm3.insert("West Indies", 500);
    hm3.insert("New Zealand", 300);
    println!("Hashmap3 = {:?}", hm3);
    hm3.remove("New Zealand");
    println!("Hashmap3 = {:?}", hm3);
}

/** Strings are implemented as a collection of bytes.
Rust has only one string type in the core language,
which is the string slice *str* that is usually seen in its borrowed form *&str*.
The *String* type, which is provided by Rust’s standard library rather than coded into the core language,
is a growable, mutable, owned, UTF-8 encoded string type.
Both String and string slice are UTF-8 encoded.

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.
A string slice is a reference to some UTF-8 encoded string data stored elsewhere or
A string slice is a reference to part of a String.

We create slices using a range within brackets by specifying [starting_index..ending_index],
where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
Internally, the slice data structure stores the starting position and the length of the slice,
which corresponds to ending_index minus starting_index.

String literals stores inside the binary. string literals are immutable; &str is an immutable reference.
*/
fn _cc_string() {
    let mut s = String::new();
    s.push_str("Hello");
    println!("{s}");
    s.pop();
    println!("{s}");
    s.push('o');

    let s2 = String::from(" Rust");

    // Value of s moved to s3
    let s3 = s + &s2;

    println!("{s3}");
}

/** vectors store their data on the heap.
 */
fn _cc_vec() {
    let mut v: Vec<&str> = Vec::new();
    v.push("Red");
    v.push("Blue");
    v.push("Green");
    println!("Size: {}", v.len());

    println!("Before popping: {:?}", v);
    v.pop();
    println!("After popping: {:?}", v);
    v.push("Green");
    v.push("Yellow");
    v.push("Turquoise");
    println!("{:?}", v);

    for entry in v {
        println!("Entry: {}", entry);
    }
}