/**
* Matrix utility functions
**/
use crate::lib::matrix::Matrix;

pub fn from(arr: Vec<Vec<f64>>) -> Matrix {
    /*
    * Create a matrix from a 2D array
    */
    Matrix::from(arr)
}

pub fn multiply(a: Matrix, b: &Matrix) -> Matrix {
    /*
    * Multiply two matrices together if the columns of A
    * match the rows of B
    */

    if a.cols != b.rows {
        panic!("Columns of A must match rows of B.");
    }

    let mut result = Matrix::new(a.rows, b.cols);

    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = 0.0;
            for k in 0..a.cols {
                sum += a.data[i][k] * b.data[k][j];
            }
            result.data[i][j] = sum;
        }
    }

    result
}

pub fn add(a: Matrix, b: Matrix) -> Matrix {
    /*
    * Add two matrices together if they have the same
    * number of rows and columns
    */

    if a.rows != b.rows || a.cols != b.cols {
        panic!("Matrices must have the same number of rows and columns.");
    }

    let mut result = Matrix::new(a.rows, a.cols);

    for i in 0..a.rows {
        for j in 0..a.cols {
            result.data[i][j] = a.data[i][j] + b.data[i][j];
        }
    }

    result
}

pub fn dot_multiply(a: Matrix, b: Matrix) -> Matrix {
    /*
    * Multiply two matrices together if they have the same
    * number of rows and columns
    */

    if a.rows != b.rows || a.cols != b.cols {
        panic!("Matrices must have the same number of rows and columns.");
    }

    let mut result = Matrix::new(a.rows, a.cols);

    for i in 0..a.rows {
        for j in 0..a.cols {
            result.data[i][j] = a.data[i][j] * b.data[i][j];
        }
    }

    result
}

pub fn subtract(a: Matrix, b: Matrix) -> Matrix {
    /*
    * Subtract two matrices together if they have the same
    * number of rows and columns
    */

    if a.rows != b.rows || a.cols != b.cols {
        panic!("Matrices must have the same number of rows and columns.");
    }

    let mut result = Matrix::new(a.rows, a.cols);

    for i in 0..a.rows {
        for j in 0..a.cols {
            result.data[i][j] = a.data[i][j] - b.data[i][j];
        }
    }

    result
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let m = from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 2);
        assert_eq!(m.data, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    }

    #[test]
    fn test_multiply() {
        let a = from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = from(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let c = multiply(a, &b);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
    }

    #[test]
    fn test_add() {
        let a = from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = from(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let c = add(a, b);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
    }

    #[test]
    fn test_dot_multiply() {
        let a = from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = from(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let c = dot_multiply(a, b);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![vec![5.0, 12.0], vec![21.0, 32.0]]);
    }

    #[test]
    fn test_subtract() {
        let a = from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = from(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let c = subtract(a, b);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![vec![-4.0, -4.0], vec![-4.0, -4.0]]);
    }
}