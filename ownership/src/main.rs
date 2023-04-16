// fn main() {
//     let mut s = String::from("hello");

//     println!("{s}");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{s}"); // this will print `hello, world!`
// }

fn main() {
    let x = 5;
    let y = x;

    println!("{x} = x, {y} = y");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    // let s2 = &s1;

    println!("{s1}, world!");
    print!("{s1} = s1, {s2} = s2");
}

// fn main() {
//     let mut s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//     s = String::from("hi");                                   // ... and so is no longer valid here
//     takes_ownership(s);
//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//     // let x = 8;                                // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. However, because s's value was moved,
//   // nothing special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//      println!("{s1}");                                   // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope
//     println!("{s2}");
//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//     println!("{s3}");                                    // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {        // gives_ownership will move its
//                                         // return value into the function
//                                         // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
//     print!("{s}");
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &r1;

//     println!("{r1}, {r2}");
// }

// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//         print!("{r1}");
//     } // r1 goes out of scope here, so we can make a new reference with no problems


// let r2 = &mut s;
// print!("{r2}");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{r1}, {r2}, and {r3}");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{r1} and {r2}");
//     // Variables r1 and r2 will not be used after this point.

//     let r3 = &mut s; // no problem
//     println!("{r3}");
// }

// fn main() {
//     let reference_to_nothing = dangle();
//     print!("{reference_to_nothing}");
// }

// fn dangle() -> String {
//     let s = String::from("hello");

//     s
// }

// fn main() {
//     // let mut s = String::from("hello world");
//     let s = "Helllo, World!";

//     let word = first_word(&s); // word will get the value 5

//     // s.clear(); // this empties the String, making it equal to ""
//     println!("the first word is: {word}");
//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
  
//       for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//               return &s[0..i];
//           }
//       }
  
//     &s[..]
//   }
// }

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial
    // or whole.
    let word = first_word(&my_string[0..6]);

    print!("{word}");

    let word = first_word(&my_string[..]);

    print!("{word}");
    // `first_word` also works on references to `String`s, which
    // are equivalent to whole slices of `String`s.
    let word = first_word(&my_string);

    print!("{word}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals,
    // whether partial or whole.
    let word = first_word(&my_string_literal[0..6]);

    print!("{word}");

    let word = first_word(&my_string_literal[..]);

    print!("{word}");
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    print!("{word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
  
      for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
              return &s[0..i];
          }
      }
  
    &s[..]
  }

// fn main() {

//     let a = [1,2,3,4,5];

//     // let slice = &a[1..3];

//     let slice = &a;

//     // assert_eq!(slice, &[2, 3]);

//     // println!("{:?}", slice);

//     assert_eq!(slice, &[1,2,3,4,5]);
//     println!("{:?}", slice);
// }