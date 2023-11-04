/*
    Clojures are anonymous functions that can:
     - Be saved in a variable
     - Capture their environment
     - Be passed as argument to other methods or functions
     - infer type annotation at compile time
     - Borrow immutable or mutable references, they can also take ownership using the move keyword
*/

use std::thread;

fn main() {
    let lucky_number = 8;

    // here the variable x is compared to lucky_number, which is a i32 thus, the compiler knows which type x is.
    // It also knows which type the clojure returns, which is a boolean.
    let validate = |x| x == lucky_number;

    assert!(!validate(1));
    assert!(validate(8));

    let invalid_type_annotation = |x| x;

    assert_eq!(
        String::from("ok"),
        invalid_type_annotation(String::from("ok"))
    );
    // Type mismatch [E0308] expected `String`, but found `i32`
    // invalid_type_annotation(23);

    let list = vec![1, 2, 3];
    println!("list is created: {:?}", list);
    let only_borrows = || println!("list from only_borrows clojure: {:?}", list);
    println!(
        "Here we have a immutable borrow from the clojure which is allowed: {:?}",
        list
    );

    only_borrows();
    println!("After calling clojure: {:?}", list);

    let mut list = list;
    let mut borrows_mutably = || list.push(7);

    // We can't have a immutable reference to list while borrows_mutably isn't called;
    // Error: immutable borrow occurs on line 36
    // println!("list while borrow mut is alive: {:?}", list);

    borrows_mutably();
    println!("after pushing: {:?}", list);

    // The clojure can take ownership using the move keyword
    // Useful to pass data to a thread, so the data won't be cleaned up before the thread finishes the execution

    let list = list;
    println!("Here is the list before moving ownership: {:?}", list);

    thread::spawn(move || println!("ownership of list: {:?}", list))
        .join()
        .unwrap();

    println!("the list is not valid in this scope anymore");
    println!("Success");

    // The way that clojures capture and handle their variables tells which kind of clojures they are
    // They can be:
    //  -> FnOnce takes ownership of the value it uses. can be called only once
    //  -> Fn does not take ownership of captured values, only borrow immutable reference
    //  -> FnMut does not take ownership of captured values, only borrow as mutable reference.
    // Useful to tell a struct or a function what clojure they can use or receive as argument.
}
