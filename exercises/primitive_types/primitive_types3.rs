// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is. 
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    // Declaring array is plain dead simple
    // [ data type; size of array ] = [ ele1, ele2, ele3 ... and so on ]
    let a = [5; 101];  // This creates array of 5 with length 101
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
