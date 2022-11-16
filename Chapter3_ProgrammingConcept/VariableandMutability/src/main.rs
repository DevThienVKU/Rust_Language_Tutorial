fn main() {

    //can change the value when mut is used
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Constants
    const THREE_HOURS_IN_SENCOND: u32 = 60 * 60 * 3;

    //Shodowning: Can declare a new variable with the same name
    // The second variable overshadows the first.
    //Can uses of the variable name to itsef until is shadowed or the scope ends.

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //Difference between mut and shadowing
    //can change the type of the value if use let
    let spaces = "   ";
    let spaces = spaces.len();
    
    println!("The number of space: {spaces}");

}
