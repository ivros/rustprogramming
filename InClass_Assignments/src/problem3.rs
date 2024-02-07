//Problem#3

//Write a modified sum function that takes a mutable reference 
//for the destination of the sum from low to high.

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    let sum = &mut *total;
    *sum = high * (high + low + 1) / 2;
}

fn main(){
    // create necessary variables and test your function for low 0 high 100 total should be 5050
    let mut total = 0;
    let low = 0;
    let high = 100;
    sum(&mut total, low, high);

    println!("{}", total);
}
