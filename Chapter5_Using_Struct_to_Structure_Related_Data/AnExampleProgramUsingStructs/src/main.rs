#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    //Refactoring with Tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    //Refactoring with Structs: Adding more meaning
    let rect1 = Rectangle {
        width: 30,
        height: 10,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(&rect1)
    );

    //Adding Useful Functionality with Derived Traits
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
