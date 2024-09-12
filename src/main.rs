use nalgebra::{Matrix2, Vector2};

fn main() {
    // Define the coefficient matrix A
    let a = Matrix2::new(4.0, 5.0,
                         2.0, 7.0);
    
    // Define the result vector b
    let b = Vector2::new(322900.0, 270980.0);
    
    // Solve for x using matrix inversion or other method
    match a.try_inverse() {
        Some(a_inv) => {
            let x = a_inv * b;
            println!("Solution: x = {}, y = {}", x[0], x[1]);
        }
        None => println!("The matrix is singular and cannot be inverted."),
    }
}
