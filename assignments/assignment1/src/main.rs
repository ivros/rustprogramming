use crate::ownerborrow::add_and_multiply;
use crate::ownerborrow::is_five;
use crate::ownerborrow::swap;

mod ownerborrow;

fn main(){

    let name = "Ivan Rosas";
    let course = "CS-3334-01";

    println!("Assignment: Ownership and Borrowing in Rust");
    println!("{}",name);
    println!("{}",course);

    // Problem 1
    let value: i32 = 5;
    let ref_value = &value;
    if is_five(ref_value){
        println!("This points to the value 5.");
    }
    else{
        println!("Is not of value 5.");
    }

    // Problem 2
    let mut value_a: i32 = 5;
    let mut value_b:i32 = 10;
    println!("Before Swap: {} {}", value_a, value_b);
    swap(&mut value_a, &mut value_b);
    println!("Swapped Values: {} {}", value_a, value_b);

    // Problem 3
    let a:i32 = 5;
    let mut b:i32 = 10;    
    add_and_multiply(&a, &mut b);
    println!("Value: {}", &mut b);

    
}
