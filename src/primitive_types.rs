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
    let s1 = "Krishnakumar";
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

        // Is empty?
        {
            assert!(!s1.is_empty(), "S1 is empty");
        }

    }


}