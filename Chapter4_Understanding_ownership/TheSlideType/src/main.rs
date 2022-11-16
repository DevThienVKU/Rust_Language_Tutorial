fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    //String Slices: A string slice is a reference to part of a String
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
    
    //another solution; 
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    //String literals are slices &str
    let s = "Hello, world!";

    //String Slices as Parameters (String and str)
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    //Other Slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}

fn first_word(s: &String) -> &str {
    //convert String to an array of bytes using as_byte method
    let bytes = s.as_bytes();

    //create an iterator over the array of bytes using iter
    //iter(): (id, reference element of array)
    //enumerate(): return a tuple
    for (i, &item) in bytes.iter().enumerate() {
        //find byte represent the space
        //the byte literal syntax 
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
