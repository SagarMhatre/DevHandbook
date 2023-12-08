fn main() {
    println!("Hello, world!");

    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    let five_hundred = tup.0;

    println!("The value of y is: {y}");
    println!("The value of x is: {five_hundred}");

    // write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5]; // = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = b[1];

    println!("{first} {second}");

    let t = triple(b[3]);

    println!("The value of t is: {t}");

    //let some_number = b[5]; // Compile error : index out of bounds: the length is 5 but the index is 5

    // Calling a function is an expression. Calling a macro is an expression.
    // A new scope block created with curly brackets is an expression, for example:

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    ctrl_statements();
    ownership();
    passing_ref();

    // [ Skipped ]  slices : https://doc.rust-lang.org/stable/book/ch04-03-slices.html

    structs_explained();

    oops_in_rust();

    error_handling();
}

fn triple(x: i32) -> i32 {
    println!("input : {x}");
    x * 3 // ; remove the semicolon to return this value - statements don’t evaluate to a value
}

fn ctrl_statements() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // Note : there is no semicolon in the braces , so it returns the value
    println!("The value of number is: {number}");
    /*
    blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.
    In this case, the value of the whole if expression depends on which block of code executes.
    This means the values that have the potential to be results from each arm of the if must be the same type;
     */
    // let number = if condition { 5 } else { "six" }; // Compiler error : expected integer, found `&str`

    // Loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter; // This also returns the value of counter
        }
    };

    println!("The result is {result}");

    /*
    If you have loops within loops, break and continue apply to the innermost loop at that point.
    You can optionally specify a loop label on a loop that you can then use with break or continue to specify
     that those keywords apply to the labeled loop instead of the innermost loop.
    Loop labels must begin with a single quote.
     */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loop
    println!("While loop ... ");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    // Slow loop
    /*

        let mut index = 0;

        while index < 5 {
            println!("the value is: {}", a[index]);

            index += 1;
        }
    */

    // For loop - faster
    println!("for loop ... ");
    for element in a {
        println!("the value is: {element}");
    }

    println!("for loop with range reversed ... "); // Prints from 4 to 1 ( does not print 5 & 0 )
    for number in (1..5).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

    // Concept : Move
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // value borrowed here after move
    println!("{}, world!", s2);

    // Rust will never automatically create “deep” copies of your data.
    // Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    // Concept : clone

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Concept : types such as integers that have a known size at compile time are stored entirely on the stack,
    // so copies of the actual values are quick to make.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    /*
    what types implement the Copy trait?
     - All the integer types, such as u32.
     - The Boolean type, bool, with values true and false.
     - All the floating-point types, such as f64.
     - The character type, char.
     - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
     */

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...

    // println!("s = {s}"); // ... and so is no longer valid here
    // Compiler Error : value borrowed here after move

    let mut x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
    x = x + 1; // but i32 is Copy, so it's okay to still
    println!("x = {x}"); // use x afterward

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length) // returning a tuple
}

fn passing_ref() {
    // A reference is like a pointer in that it’s an address we can follow to access the data stored at that address;
    // that data is owned by some other variable.
    // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

    let mut s1 = String::from("hello sagar");
    let len = calculate_length2(&s1); // Put & when calling & in defintion
    println!("The length of '{}' is {}.", s1, len);

    change(&s1);

    // Mutable Reference
    change2(&mut s1);

    println!("The value  of s1 is '{}'", s1); // 'hello sagar, world'

    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;    //Compiler error : cannot borrow `s` as mutable more than once at a time
    // println!("{}, {}", r1, r2);
    println!("r1 = {} ", r1);

    /*
    Data Race

    A data race is similar to a race condition and happens when these three behaviors occur:

        Two or more pointers access the same data at the same time.
        At least one of the pointers is being used to write to the data.
        There’s no mechanism being used to synchronize access to the data.
     */

    //  use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

    let mut s = String::from("hello braces ");

    {
        let r3 = &mut s;
    } // r3 goes out of scope here, so we can make a new reference with no problems.

    let r4 = &mut s;

    //println!("{}, {}", r3, r4); //  cannot find value `r3` in this scope

    println!("r4 = {} ", r4);

    // Rust enforces a similar rule for combining mutable and immutable references.

    let mut s = String::from("hello");

    let r5 = &s; // no problem  - immutable
    let r6 = &s; // no problem  - immutable
                 // let r7 = &mut s; // Compiler Error : cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("r5 = {},  r6 = {} ", r5, r6);

    // a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
    // this code will compile because the last usage of the immutable references, the println!,
    // occurs before the mutable reference is introduced:

    let mut s = String::from("hello");

    let r8 = &s; // no problem
    let r9 = &s; // no problem
    println!("moved to println! r8 = {} and  r9 = {}", r8, r9);
    // variables r1 and r2 will not be used after this point

    let r10 = &mut s; // no problem
    println!("r10 = {}", r10);

    //let reference_to_nothing = dangle();
    let some_string = no_dangle();
    println!("some_string = {}", some_string);
}

fn calculate_length2(s: &String) -> usize {
    // s is a reference to a String // Put & when calling & in defintion
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &String) {
    // some_string.push_str(", world"); // Compiler Error :  `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

// we create a mutable reference with &mut
fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String { // dangle returns a reference to a String Compiler error : expected named lifetime parameter

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
// But we tried to return a reference to it. That means this reference would be pointing to an invalid String.

// The solution here is to return the String directly instead of the reference
fn no_dangle() -> String {
    let s = String::from("hello from no_dangling !!!");

    s
}

// struct - https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html

//  we have to explicitly opt in to make that functionality to print out debugging information, available for our struct.
//  To do that, we add the outer attribute #[derive(Debug)] just before the struct definition
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Constructur (sort of :) )
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        /*
        username: username,
        email: email,
        // .. works, but inconvenient
        */
        username,
        email,
        // ... convenient la !!!
        sign_in_count: 1,
    }
}

fn structs_explained() {
    // Examaple 1  --------------------------------------
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        // email: "someone@example.com",   // Compiler error : expected `String`, found `&str`
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user1 : {:?} ", user1);

    // Example 2  --------------------------------------

    // user1.email = String::from("anotheremail@example.com");
    // cannot assign to `user1.email`, as `user1` is not declared as mutable

    // entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user2 = User {
        active: true,
        username: String::from("someusername2"),
        // email: "someone@example.com",   // Compiler error : expected `String`, found `&str`
        email: String::from("someone2@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    println!("user2 : {:?} ", user2);

    // Example 3  --------------------------------------

    // cannot use user1 - since it has moved to println , but can use user1.active

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another3@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("user3  : {:#?} ", user3);
    // When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println! string.

    // Examaple 4  --------------------------------------

    // creating user1 again since it has moved to println
    let user1 = User {
        active: true,
        username: String::from("someusername1"),
        // email: "someone@example.com",   // Compiler error : expected `String`, found `&str`
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    let user4 = User {
        email: String::from("another4@example.com"),
        ..user1
    };
    // we can no longer use user1 as a whole after creating user3 because the String in the username field of user1 was moved into user3.
    dbg!(&user4);
    // dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout).

    // dbg! returns ownership of the expression’s valu
    println!("user4  : {:#?} ", user4);
}

fn oops_in_rust() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("sq  : {:#?} ", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Unlike functions, methods are defined within the context of a struct
    // (or an enum or a trait object),
    // and their first parameter is always self,
    // which represents the instance of the struct the method is being called on.

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods can take ownership of self, borrow self immutably, as we’ve done here,
    // or borrow self mutably, just as they can any other parameter.

    // If we wanted to change the instance that we’ve called the method on as part of what the method does,
    // we’d use &mut self as the first parameter.

    // we can choose to give a method the same name as one of the struct’s fields.
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // ( Java static methods )
    // We can define associated functions that don’t have self as their first parameter (and thus are not methods)
    // because they don’t need an instance of the type to work with.

    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn error_handling() {
    // https://learn.microsoft.com/en-us/training/modules/rust-error-handling/1-introduction

    // panic!("Farewell!"); // program would exit with status code 101

    // use panic! when a program reaches an unrecoverable state. A state where there's absolutely no way to recover from the error.

    /*
    // export RUST_BACKTRACE=1
    // set RUST_BACKTRACE=1
    Farewell!
        stack backtrace:
        0: std::panicking::begin_panic_handler
                    at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library\std\src\panicking.rs:595
        1: core::panicking::panic_fmt
                    at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library\core\src\panicking.rs:67
        2: hello_world_rust::error_handling

     */

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    //  If the value exists at a specified index, it's wrapped in the Option::Some(value) variant. If the index is out of bounds, it would return an Option::None value instead.

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {}
    }

    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    } else {
        println!("Not my fav number!");
    }

    let a_number: Option<u8> = Some(6);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    } else {
        println!("Not my fav number!");
    }
    
}
