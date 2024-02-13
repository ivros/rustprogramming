// Problem 1: Is Five
// Write a function that checks if a given reference to an i32 points to the value 5.
pub fn is_five(x: &i32) -> bool {
    // Your implementation goes here
    let value:i32 = 5;

    *x == value
}


// Problem 2: Swap Two Numbers
// Write a function swap that takes two mutable references to i32 and swaps their values. Do not use the standard library's swap method.
pub fn swap(a: &mut i32, b: &mut i32) {
    // Implement this function
    let c:i32 = *a;
    *a = *b;
    *b = c;
}

// // Problem 3: Immutable and Mutable References
// // Write a function add_and_multiply that takes an immutable reference to an i32 and a mutable reference to another i32. The function should add the first parameter to the second and then multiply the result by 2. Return the new value of the second parameter.
pub fn add_and_multiply(a: &i32, b: &mut i32) -> i32{
    // Implement this function
    let sum = *a + *b;
    let result: i32 = sum * 2;
    *b = result;
    *b
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_five() {
        
        assert!(is_five(&5), "Expected true when the value is 5");
        assert!(!is_five(&6), "Expected false when the value is not 5");
    }
    

    #[test]
    fn test_swap() {
        let mut x = 5;
        let mut y = 10;
        swap(&mut x, &mut y);
        assert_eq!(x, 10);
        assert_eq!(y, 5);
    }


    #[test]
    fn test_add_and_multiply() {
        let a = 10;
        let mut b = 20;
        add_and_multiply(&a, &mut b);
        assert_eq!(b, 60); // (20 + 10) * 2
    }
}