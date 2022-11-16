struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email = email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//Using the Field Init Shorthand
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
//Using Tuple Structs without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-Like Structs Without Any Fields
struct AlwaysEqual;

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }

    user1.email = String::from("anotheremail@example.com");

    //Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }

    //struct update syntax (another version)
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    }

    //Using Tuple Structs without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //Unit-Like Structs Without Any Fields
    let subject = AlwaysEqual;

}
