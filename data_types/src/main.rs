fn main() {
    let guess: i32 = "42".parse().expect("Not a number!");
    let number = 3.14;
    let is_ready = true;
    let letter = 'R';
    
    println!("Guess: {}, Number: {}, Ready: {}, Letter: {}", 
             guess, number, is_ready, letter);
}