// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
  // Returns a transposed a 3x3 matrix
  let mut transposed = [[0; 3]; 3];
  for col in 0..3 {
    for row in 0..3 {
      transposed[col][row] = matrix[row][col];
    }
  }
  return transposed;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
  // Prints a 3x3 matrix in latex
  println!("\\begin{{bmatrix}}");
  for col in 0..3 {
    for row in 0..3 {
      if row < 2 {
        print!("{}& ", matrix[col][row]);
      } else {
        println!("{}\\\\ ", matrix[col][row]);
      }
    }
  }
  println!("\\end{{bmatrix}}");
}

fn main() {
  let matrix = [
    [1, 2, 3], // <-- the comment makes rustfmt add a newline
    [4, 5, 6],
    [7, 8, 9],
  ];

  println!("matrix:");
  pretty_print(&matrix);

  let transposed = transpose(matrix);
  println!("\ntransposed:");
  pretty_print(&transposed);
}