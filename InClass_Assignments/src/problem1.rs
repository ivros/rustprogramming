// In-Class Practice

// Warm-up
// Problem #1
// String Concatenation with Borrowing:

// Write a function that concatenates two strings without taking ownership, i.e., by borrowing.

fn concat_strings(s1: &String, s2: &String) -> String {
    format!("{}{}", *s1, *s2)
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}