mod integer_range;
use crate::integer_range::fits_in_type;

fn main() {
    let guess:i32 = "42".parse().expect("Not a number!");
    let number = 3.14;
    let is_ready = true;
    let letter = 'R';
    
    println!("Guess: {}, Number: {}, Ready: {}, Letter: {}", 
             guess, number, is_ready, letter);
    // println!("Does 300 fit in i8  ?{} ", fits_in_type(100, "i8"));  
}