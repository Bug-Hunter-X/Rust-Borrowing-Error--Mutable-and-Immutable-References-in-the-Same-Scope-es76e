fn main() {
    let mut x = 5;
    {
        let y = &mut x; // Mutable reference in an inner scope
        *y = 10;  // Modify x through y
    }
    let z = &x; // Immutable reference in a separate scope

    println!("x = {}", x); // Output: x = 10
    println!("z = {}", z); // Output: z = 10
} 