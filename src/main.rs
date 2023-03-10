extern crate core;

mod countries;
mod primitive_types;
mod collections;
mod error_handling;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use countries::powerful_countries::NuclearPowers;

fn main() {
    println!("main() >>");
    //perform_basic_rust_operations();
    //let squared = calculate_square(5);
    //println!("Squared value = {squared}");

    //_control_flow();
    //_references_and_borrowing();
    //let sentence : String = String::from("My name is Krishnakumar");
    //_find_my_name_in_string(sentence, "Krishnakumar");

    //_work_on_struct();

    //collections::_common_collections();
    //_generics();
    //_traits();
    //_lifetimes();

    //_copy_from_file();

    //primitive_types::pt_str();
    error_handling::_error_handling();
    println!("<< main()");
}

fn _lifetimes() {
    let s1 = String::from("Krishna");
    let s2 = String::from("Krishna");

    let p1 = s1.as_ptr();
    println!("{:?}", p1);
    let p2 = s2.as_ptr();
    println!("{:?}", p2);
}


fn _copy_from_file() {
    // Read from file1 and write to file2
    let path1 = Path::new("resources/file1.txt");
    let path2 = Path::new("resources/file2.txt");

    let mut file1 = File::open(path1).unwrap();
    let mut contents1 = String::new();
    let size = file1.read_to_string(&mut contents1).unwrap();
    println!("{} has {} letters", contents1, size);

    let mut file = File::create(path2).unwrap();
    file.write_all(contents1.as_bytes()).expect("Error found while writing");
}

fn _traits() {

    let c1 = Country::new(String::from("Saudi Arabia"));
    println!("{}", c1);
    let f = c1.get_fund_from_Europe();
    println!("{}", f);

}

trait USDTransactor {
    fn new(name: String) -> Self;
}

trait EURTransactor {
    fn get_fund_from_Europe(&self) -> i32;
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, currency: {}", self.name, self.currency)
    }
}

impl USDTransactor for Country {
    fn new(name: String) -> Self {
        Country { name, currency: String::from("USD")}
    }
}

impl EURTransactor for Country {
    fn get_fund_from_Europe(&self) -> i32 {
        1000000
    }
}


fn _generics() {




}


fn _work_on_struct() {
    let mut c1 : Country = Country {
        name: String::from("Sri Lanka"),
        currency: String::from("Rupee")
    };
    println!("Before update: c1 = {:?}", c1);

    c1.currency = String::from("USD");

    println!("After update: c1 = {:?}", c1);


    let c2 : Country = Country {
        name: String::from("Bangladesh"),
        ..c1
    };
    println!("c1 partially moved its values to c2 = {:?}", c2);

    //Calling method
    let n1 : &str = c2.get_currency();
    println!("Currency of c2: {:?}", n1);

    let capital = countries::get_country_capital(NuclearPowers::India);
    println!("{}", &capital);
}

#[derive(Debug)] #[allow(dead_code)]
struct Country {
    name: String,
    currency:String,
}

// Methods and Associated functions
impl Country {
    fn get_currency(&self) -> &str {
         &self.currency
    }
}


// String slices
fn _find_my_name_in_string(sentence: String, find: &str) {

    println!("Finding {} in the sentence : {}", find, sentence);

    let bytes_of_string = sentence.as_bytes();
    let tuple_of_byte = bytes_of_string.iter().enumerate();

    for (index, &to_byte) in tuple_of_byte {
        unsafe {
            println!("The index is {} and the reference is {}", index, std::str::from_utf8_unchecked(&[to_byte]));
        }
        if to_byte == b' ' {
            println!("Index of space: {}", index);
            let string_slice = &sentence[0..index];
            println!("String slice: {}", string_slice);
        }
    }
}

// Ownership rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn _references_and_borrowing() {

    {
        // When a variable comes into scope, it is valid. It remains valid until it goes out of scope.
        let _a = 0; // s is not valid here, it???s not yet declared
        let mut s = 1; // s is valid from this point forward
        s +=1;
        println!("s={}", s);
    } // this scope is now over, and s is no longer valid


    // At any point of time, we can have either one mutable reference or any number of immutable references.
    // References must always be valid.
    let x = 10;
    // We can have many immutable references.
    let x1 = &x;
    let x2 = &x;
    println!("The x is {} and the immutable references of x are {}, {}", x, x1, x2);

    let mut y = 4;
    // We can have only one mutable reference.
    let y1 = &mut y;
    *y1 += 1;
    println!("The mutable reference of y is {}", y1);

}



fn _control_flow() {
    println!("control_flow() >>");
    // if is an expression. It can return values.

    let mut a = 0;
    // Both block's last expression must be of same type
    let status = if a == 0 {"a is zero"} else {"a is non-zero"};
    println!("status = {status}");

    // For loops within loops, break and continue apply to the innermost loop at that point.
    loop {
        a+=1;
        if a <= 3 {
            continue;
        }
        if a >= 10 {
            break;
        }
        println!("Current value of a = {a}");
    }

    // Labelled loops. Label must start with single quote.
    let mut c = 0;
    'loop1: loop {
        c+=1;
        println!("c = {c}");

        let mut d = 0;
        'loop2: loop {
            d+=1;
            println!("d = {d}");
            let cd = c * d;
            println!("cd = {cd}");
            if d>=5 { break 'loop2; }

            if cd == 16 { break 'loop1; };
        }

        if c >= 5 { break; };

    }

    let mut e = 0;

    while e < 3 {
        println!("The number {} is less than 3", e);
        e+=1;
    }

    let arr = [5,10,15,20,25,26,35,40];

    for element in arr {
        if element%5 != 0 {
            println!("The element {element} in not a multiple of 5");
        }
    }

    let country:NuclearPowers = NuclearPowers::India;

    match country {
        NuclearPowers::India => println!("It's India"),
        _ => println!("It's not India")
    }

    println!("<< control_flow()");
}



// Expression based return without return keyword;
fn _calculate_square(x: i64) -> i64 {

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.

    let b = {
        let b = 2; // This is a statement.

        // Expressions do not include ending semicolons.
        // If you add a semicolon to the end of an expression, you turn it into a statement

        b*x // This is an expression.
    };

    println!("Twice of {x} = {b}");

    x*x
}


fn _perform_basic_rust_operations() {

    // Variables are immutable by default
    let a = 1;
    println!("a = {}", a);
    {
        // Shadowing within this inner scope
        let a = 10;
        println!("while shadowing: a = {}", a);
    }
    println!("after shadowing: a = {}", a);


    // Variables can be set as mutable
    let mut b = 2;
    println!("b = {}", b);
    b=3;
    println!("after reassignment: b = {}", b);

    // Constants are not only immutable by default but always immutable
    const MAX_NUM_OF_THREADS: i32 = 200;

    // Rust have scalar (integer, floating point number, boolean, character) and compound data types
    const SIGNED_INTEGER_8BIT: i8 = 8; // -128 to 127
    const SIGNED_INTEGER_16BIT: i16 = 16;
    const SIGNED_INTEGER_32BIT: i32 = 32;
    const SIGNED_INTEGER_64BIT: i64 = 64;
    const SIGNED_INTEGER_128BIT: i128 = 128;
    const SIGNED_INTEGER_ARCHITECTURE: isize = 32 * 64;

    const UNSIGNED_INTEGER_8BIT: u8 = 8;
    const UNSIGNED_INTEGER_16BIT: u16 = 16;
    const UNSIGNED_INTEGER_32BIT: u32 = 32;
    const UNSIGNED_INTEGER_64BIT: u64 = 64;
    const UNSIGNED_INTEGER_128BIT: u128 = 128;
    const UNSIGNED_INTEGER_ARCHITECTURE: usize = 32 * 64;

    {
        // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
        let t1 = (1,2,3);
        // Tuples can store different data type
        let t2 : (i8, u8, f32) = (127, 255, 1.65);

        // Tuple can't be formatted directly for print
        println!("Tuple t2 = {:?}", t2);

        // Destructuring Tuple
        let (a,b,c) = t1;
        println!("a={}, b={}, c={}",a,b,c);

        // Accessing tuple elements based on index
        let t2_0 = t2.0;
        let t2_1 = t2.1;
        let t2_2 = t2.2;

        println!("t2_0={}, t2_1={}, t2_2={}",t2_0,t2_1,t2_2);
    }

    // An array is a single chunk of memory on the stack rather than the heap
    {
        // Unlike Tuple, Array can't store different data type
        let a1 = [1,4,56,77];

        // Like Tuple, Array have a fixed length: once declared, they cannot grow or shrink in size.
        let a2 : [i32;5] = [3,5,6,7,8];
        // Vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

        // Access elements using indices
        let index0 = a1[0];
        let index2 = a2[2];
        println!("{} {}", index0, index2);

    }

}