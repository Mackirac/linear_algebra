use std::ops::{ Index, IndexMut, Add, Sub, Mul };
use std::fmt::{ self, Formatter, Debug };
use crate::Number;

pub struct Matrix <T> {
    rows : usize,
    cols : usize,
    vals : Box<[Box<[T]>]>
}

impl <T> Index<(usize, usize)> for Matrix <T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 == 0 || index.1 == 0 { panic!("Index out of bonds") }
        &self.vals[index.0 - 1][index.1 - 1]
    }
}
impl <T> IndexMut<(usize, usize)> for Matrix <T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 == 0 || index.1 == 0 { panic!("Index out of bonds") }
        &mut self.vals[index.0 - 1][index.1 - 1]
    }
}

impl <T> Matrix <T> {
    pub fn rows(&self) -> usize { self.rows }
    pub fn cols(&self) -> usize { self.cols }
    pub fn dims(&self) -> (usize, usize) { (self.rows, self.cols) }
    pub fn is_square(&self) -> bool { self.rows == self.cols }

    pub fn new(rows: usize, cols: usize, f: impl Fn(usize, usize)->T) -> Matrix<T> {
        if rows == 0 || cols == 0 { panic!("Invalid matrix dimensions") }
        let mut vals = Vec::with_capacity(rows);
        for l in 1..rows+1 {
            let mut row = Vec::with_capacity(cols);
            for c in 1..cols+1 {
                row.push(f(l, c));
            }
            vals.push(row.into_boxed_slice());
        }
        let vals = vals.into_boxed_slice();
        Matrix { rows, cols, vals }
    }
}

impl <T: Number> Matrix <T> {
    pub fn zeros(rows: usize, cols: usize) -> Matrix<T> {
        Matrix::new(rows, cols, |_, _| { T::zero() })
    }
}

impl <T: Clone> Matrix <T> {
    pub fn from_vec(rows: usize, cols: usize, vals: Vec<T>) -> Matrix<T> {
        Matrix::new(rows, cols, |l, c| { vals[(l-1)*cols + c - 1].clone() })
    }

    pub fn repeat(val: T, rows: usize, cols: usize) -> Matrix <T> {
        Matrix::new(rows, cols, |_, _| { val.clone() })
    }
}

impl <T, U, V> Add<Matrix<U>> for Matrix <T> where
    T: Add<U, Output=V> + Clone,
    U: Clone
{
    type Output = Matrix<V>;
    fn add(self, other: Matrix<U>) -> Self::Output {
        if self.dims() != other.dims() { panic!("Unmatching matrices length") }
        Matrix::new(self.rows, self.cols, |l, c| {
            self[(l, c)].clone() + other[(l, c)].clone()
        })
    }
}

impl <T, U, V> Sub<Matrix<U>> for Matrix <T> where
    T: Sub<U, Output=V> + Clone,
    U: Clone
{
    type Output = Matrix<V>;
    fn sub(self, other: Matrix<U>) -> Self::Output {
        if self.dims() != other.dims() { panic!("Unmatching matrices length") }
        Matrix::new(self.rows, self.cols, |l, c| {
            self[(l, c)].clone() - other[(l, c)].clone()
        })
    }
}

impl <T, U, V> Mul<Matrix<U>> for Matrix <T> where
    T: Mul<U, Output=V> + Clone,
    U: Clone,
    V: Number + Add<V, Output=V>
{
    type Output = Matrix<V>;
    fn mul(self, other: Matrix<U>) -> Self::Output {
        if self.cols != other.rows { panic!("Unmatching matrices length") }
        Matrix::new(self.rows, other.cols, |l, c| {
            let mut val = V::zero();
            for _c in 1..self.cols+1 {
                val = val + self[(l, _c)].clone() * other[(_c, c)].clone();
            }
            val
        })
    }
}

impl <T, U, V> Mul<U> for Matrix <T> where
    T: Clone,
    U: Mul<T, Output=V> + Clone
{
    type Output = Matrix<V>;
    fn mul(self, scalar: U) -> Self::Output {
        Matrix::new(self.rows, self.cols, |l, c| { scalar.clone() * self[(l, c)].clone() })
    }
}

impl <T: Debug> Debug for Matrix <T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for l in 1..self.rows+1 {
            for c in 1..self.cols+1 {
                buffer.push_str(&format!("{:?}   ", self[(l, c)]));
            }
        }
        write!(f, "{}", buffer)
    }
}
