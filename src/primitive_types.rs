use std::slice;
use std::str;

pub fn pt_str() {

    //   - Only a comment
    /*   - Only a comment */
    //!  - Inner line doc
    /*!  - Inner block doc */
    ///  - Outer line doc (exactly 3 slashes)
    /**  - Outer block doc (exactly) 2 asterisks */

    /** str type is called as string slice.
    String literals are string slices.
    We have declared a string slice initialized with a string literal.
    String literals have a static lifetime, which means the string is guaranteed to be valid for the duration of the entire program.
    It is usually seen in its borrowed form, &str.
    A &str is made up of two components: a pointer to some bytes, and a length.
     */
    let s1 = "Krishna Kumar";
    println!("s1 = {}", s1);


    /** Components of str */
    {
        let p1 = s1.as_ptr();
        let len1 = s1.len();
        println!("Pointer: {:?}, Length: {}", p1, len1);

        // Building new string with pointer and length, which is unsafe.

        let raw_parts = unsafe {
            let slice = slice::from_raw_parts(p1, len1);
            let s2 = str::from_utf8(slice).unwrap();
            let p2 = s2.as_ptr();
            let len2 = s2.len();
            println!("Pointer: {:?}, Length: {}", p2, len2);
            println!("unsafe s2 = {}", s2);
        };

        // Length doesn't means number of character instead number of bytes as per UTF-8.
        let s3 = "😂";
        println!("Bytes of emoji: {:?}. Length of emoji: {}", s3.as_bytes() ,s3.len());
    }

    // Is empty?
    assert!(!s1.is_empty(), "S1 is empty");
    // Contains?
    if s1.contains(" ") {
        println!("His name contains space as follows: Krishna Kumar");
    }

    if s1.starts_with("Krishna") {
        println!("His name starts with Krishna");
    }

    if s1.ends_with("Kumar") {
        println!("His name ends with Kumar");
    }

    // Get methods. Taking sub-slice of string slice.
    {
        let mut s2 = "I am learning Rust";

        /// Return immutable reference
        let s21 = s2.get(..13);
        match s21 {
            Some("I am") => println!("Matching: {}", s21.unwrap()),
            Some("I am learning") => println!("Matching: {}", s21.unwrap()),
            None => println!("None"),
            _ => println!("Not Matching")
        }

        /// Return None if index goes beyond the length
        let s22 = s2.get(..60);
        match s22 {
            Some("I am learning") => println!("Matching: {}", s22.unwrap()),
            None => println!("None"),
            _ => println!("Not Matching")
        }

        // Mutable get methods.
        {
            // New variable s2 is valid for this scope. It's called shadowing.
            let mut s2 = String::from("I am learning Rust");

            // Immutable binding to mutable reference
            let ms21 = s2.get_mut(..13).unwrap();
            // &mut str is very restrictive. It allows only very few operations as below.
            ms21.make_ascii_lowercase();
            println!("lower case: {}", ms21);
            ms21.make_ascii_uppercase();
            println!("UPPER CASE: {}", ms21); // Mutable reference goes out of scope here.

            println!("Original value: {}", &s2);
        }
    }



}