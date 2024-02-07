fn clone_and_modify(s: &String) -> String {
    let cloned_string = s.clone();
    let new_word = "World!".to_string();
    format!("{}{}", cloned_string, new_word)
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}