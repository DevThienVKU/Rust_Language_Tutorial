# What is Ownership?
Ownership is a set of rules that governs how a Rust program manages memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.
## Ownership Rules
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped
## Variable Scope
- When s comes into scope, it is valid
- It remains valid until it goes out of scope
## The String Type
The types covered previously are all a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope.
But want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the String type is a great example.
- A string value is hardcoded into our program. String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. 
- One reason is that they’re immutable
> what if we want to take user input and store it? 
- Rust has a second string type, String
- This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
- Can create a String from a string literal using the from function
## Memory and Allocation
- In the case of a string literal, know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.But these properties only come from the string literal’s immutability. 
But it's can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.
- With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. 
    - The memory must be requested from the memory allocator at runtime. 
    - We need a way of returning this memory to the allocator when we’re done with our String.

### Ways Variables and Data Interact: Move
```rust
    //can't run
    let s1 = String::from("hello");
    let s2 = s1;
```
A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity.
![image](https://doc.rust-lang.org/book/img/trpl04-01.svg)
When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.
![image](https://doc.rust-lang.org/book/img/trpl04-02.svg)
what memory would look like if Rust instead copied the heap data as well. If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large.
![image](https://doc.rust-lang.org/book/img/trpl04-03.svg)
When a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable.
This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work.
```rust
    let s1 = String::from("hello");
        let s2 = s1;

        println!("{}, world!", s1);
```
Because Rust also invalidates the first variable, instead of calling it a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2.
![image](https://doc.rust-lang.org/book/img/trpl04-04.svg)
    >Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
### Stack-Only Data: Copy
The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. 
Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are. If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.
Here are some of the types that implement Copy:
- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

## Ownership and Functions
The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

## Return Values and Scope
Returning values can also transfer ownership.

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
Rust does let us return multiple values using a tuple.
