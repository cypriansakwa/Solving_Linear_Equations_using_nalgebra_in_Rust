## Overview
This Rust program shows how to solve a system of linear equations using matrix inversion using the nalgebra library. It takes a $2\times2$ coefficient matrix and a 2-dimensional result vector, then computes the solution vector if the matrix is invertible.
## Main Functionality
- Define a $2\times 2$ matrix $A$ and a result vector $b$.
- Attempt to compute the inverse of matrix $A$ using **try_inverse()**.
- If the inverse exists, multiply it by vector $b$ to find the solution vector x.
- Print the solution or a message if the matrix is singular (non-invertible).
## Installation
- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Dependencies
The program uses the nalgebra crate for matrix operations. Ensure you have nalgebra added to your Cargo.toml:
>```
>[dependencies]
>nalgebra = "0.30"  # Replace with the latest version if needed

## Usage
- To use this code, you can clone the repository.
- You can change the values of $a$ and $b$ in the main function to test different cases.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
- The output will display the values of $x$ and $y$ that solve the linear equations, or a message stating that the matrix is singular.
## Example defining the Coefficient Matrix A
- The matrix $A$ is a $2\times 2$ matrix with elements defined as:

 $$A=\begin{pmatrix}
	4&5\\
	2&7\\
\end{pmatrix}$$
- This matrix is created using the **Matrix2::new** method..
 >```
> let a = Matrix2::new(4.0, 5.0, 2.0, 7.0);
## Example defining the Result Vector b
- The vector b is a 2-dimensional vector with elements $322900.0$ and $270980.0$
 >```
> let b = Vector2::new(322900.0, 270980.0);
## Expected Output Solution
 >```
> Solution: x = 38570.0, y = 44520.0
## Handling Errors
- If the matrix is singular the program will output
- >```
  >The matrix is singular and cannot be inverted.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/Solving_Linear_Equations_using_nalgebra_in_Rust.git
   cd Solving_Linear_Equations_using_nalgebra_in_Rust
