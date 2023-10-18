/**
 * Matrix implementation in rust
 * 
**/
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        /*
        * Create a new matrix with all values set to 0.0
        */
        
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn from(arr: Vec<Vec<f64>>) -> Matrix {
        /*  
        * Create a matrix from a 2D array
        */

        Matrix {
            rows: arr.len(),
            cols: arr[0].len(),
            data: arr,
        }
    }

    pub fn random(rows: usize, cols: usize) -> Self {
        /* Create a matrix with random values between -1 and 1 */

        let mut m = Matrix::new(rows, cols);
        let mut rng = thread_rng();

        for i in 0..m.rows {
            for j in 0..m.cols {
                m.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        m
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        /*
        * Multiply two matrices together if the columns of self
        * match the rows of other
        */
    
        if self.cols != other.rows {
            panic!("Columns of self must match rows of other.");
        }
    
        let mut result = Matrix::new(self.rows, other.cols);
    
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
    
        result
    }


    pub fn add(&self, other: &Matrix) -> Matrix {
        /*
        * Add two matrices together if they have the same
        * number of rows and columns
        */
    
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrices must have the same number of rows and columns.");
        }
    
        let mut result = Matrix::new(self.rows, self.cols);
    
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
    
        result
    }
    
    pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
        /*
        * Multiply two matrices together if they have the same
        * number of rows and columns
        */
    
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrices must have the same number of rows and columns.");
        }
    
        let mut result = Matrix::new(self.rows, self.cols);
    
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
    
        result
    }
    
    pub fn subtract(&self, other: &Matrix) -> Matrix {
        /*
        * Subtract two matrices together if they have the same
        * number of rows and columns
        */
    
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrices must have the same number of rows and columns.");
        }
    
        let mut result = Matrix::new(self.rows, self.cols);
    
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
    
        result
    }


    pub fn map(&mut self, func: &dyn Fn(f64) -> f64) -> Matrix {
        /*
        * Apply a function to every element of the matrix
        */
        Matrix::from(self.data.iter()
        .map(
            |row| row
            .iter()
            .map(|x| func(*x))
            .collect())
        .collect())
    }

    
    pub fn transpose(&self) -> Self {
        /*
        * Transpose the matrix
        */
        let mut result = Self::new(self.cols, self.rows);
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        
        result
    }


    pub fn print(&self) {
        /*
        * Print the matrix to the console
        */
        println!("[");
        for i in 0..self.rows {
            print!("[");
            for j in 0..self.cols {
                print!("{} ", self.data[i][j]);
            }
            println!("]");
        }
        println!("]");
    }
}

pub fn from(arr: Vec<Vec<f64>>) -> Matrix {
    /*  
    * Create a matrix from a 2D array
    */

    Matrix::from(arr)
}     
        
// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m0 = Matrix::new(0, 0);
        assert_eq!(m0.rows, 0);
        assert_eq!(m0.cols, 0);
        assert_eq!(m0.data, Vec::<Vec<f64>>::new());

        let m1 = Matrix::new(1, 1);
        assert_eq!(m1.rows, 1);
        assert_eq!(m1.cols, 1);
        assert_eq!(m1.data, vec![vec![0.0]]);

        let m2 = Matrix::new(2, 2);
        assert_eq!(m2.rows, 2);
        assert_eq!(m2.cols, 2);
        assert_eq!(m2.data, vec![vec![0.0, 0.0], vec![0.0, 0.0]]);

        let m3 = Matrix::new(2, 3);
        assert_eq!(m3.rows, 2);
        assert_eq!(m3.cols, 3);
        assert_eq!(m3.data, vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]]);
    }

    #[test]
    fn test_random() {
        let m = Matrix::random(2, 2);
        
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 2);
        assert_ne!(m.data, vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
    }

    #[test]
    fn test_map() {
        let mut m0 = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let r0 = m0.map(&|x| x * 2.0);
        assert_eq!(r0.data, vec![vec![2.0, 4.0], vec![6.0, 8.0]]);

        let mut m1 = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let r1 = m1.map(&|x| x + 1.0);
        assert_eq!(r1.data, vec![vec![2.0, 3.0], vec![4.0, 5.0]]);

        let mut m2 = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        fn f(x: f64) -> f64 {
            x * 2.0
        }
        let r2 = m2.map(&f);
        assert_eq!(r2.data, vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
    }

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
        let c = a.multiply(&b);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
    }

    #[test]
    fn test_add() {
        let a = from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = from(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let c = a.add(&b);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
    }

    #[test]
    fn test_dot_multiply() {
        let a = from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = from(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let c = a.dot_multiply(&b);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![vec![5.0, 12.0], vec![21.0, 32.0]]);
    }

    #[test]
    fn test_subtract() {
        let a = from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = from(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let c = a.subtract(&b);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![vec![-4.0, -4.0], vec![-4.0, -4.0]]);
    }
}