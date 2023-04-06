// fn main() {
    
//     struct User {
//         active: bool,
//         username: String,
//         email: String,
//         sign_in_count: u64,
//     }

//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("billybob@bingo.com");

//     let user2 = User {
//         email: String::from("macaroni@cheese.com"),
//         ..user1
//     };

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }
// }

// struct Color(i32,i32,i32);
// struct Point(i32,i32,i32);

// fn main() {
//     let black = Color(0,0,0);
//     let origin = Point(0,0,0);

//     println!("{:?}", black.0);
//     println!("{:?}", origin.0);
// }

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}