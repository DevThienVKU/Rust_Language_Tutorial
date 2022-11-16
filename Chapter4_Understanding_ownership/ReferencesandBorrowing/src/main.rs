fn main() {
    //Reference
    //Follow to access the data stored at that address
    //that data is owned by some other variable
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //Mutable References
    //modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:
    let mut s = String::from("hello");

    change(&mut s);

    //As always, we can use curly brackets to create a new scope, 
    //allowing for multiple mutable references, just not simultaneous ones:
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    //this code will compile because the last usage of the immutable references, 
    //the println!, occurs before the mutable reference is introduced:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    //Dangling References
    
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}