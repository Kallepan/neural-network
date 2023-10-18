/**
 * Matrix implementation in rust
 * 
**/
use rand::{thread_rng, Rng};

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

    pub fn from(arr: Vec<Vec<f64>>) -> Self {
        /*  
        * Create a matrix from a 2D array
        */

        let rows = arr.len();
        let cols = arr[0].len();
        let mut m = Matrix::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                m.data[i][j] = arr[i][j];
            }
        }

        m
    }

    pub fn random(&mut self) {
        /* modify the matrix with random values
        * between -1 and 1
        */
        let mut rng = thread_rng();

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
    }


    pub fn map(&mut self, func: fn(f64) -> f64) {
        /*
        * Apply a function to every element of the matrix
        */
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = func(self.data[i][j]);
            }
        }
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
        let mut m = Matrix::new(2, 2);
        m.random();
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 2);
        assert_ne!(m.data, vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
    }

    #[test]
    fn test_map() {
        let mut m = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        m.map(|x| x * 2.0);
        assert_eq!(m.data, vec![vec![2.0, 4.0], vec![6.0, 8.0]]);

        let mut m = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        m.map(|x| x + 1.0);
        assert_eq!(m.data, vec![vec![2.0, 3.0], vec![4.0, 5.0]]);

        let mut m = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        fn f(x: f64) -> f64 {
            x * 2.0
        }
        m.map(f);
        assert_eq!(m.data, vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
    }
}