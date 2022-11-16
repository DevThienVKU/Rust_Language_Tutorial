fn main() {
    
    //Function
    //snake case, lower case and underscores separate words
    println!("Hello World!");
    
    another_function();
    another_function2(2);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is {x}");

}

//define a function by entering fn and set of parentheses
fn another_function(){
    println!("Another function.");
}
//Parameters
fn another_function2(x: i32){
    println!("The value of x is: {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

//function with return values
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32{
    x + 1
}