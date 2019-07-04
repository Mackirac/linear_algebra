use std::ops::{ Index, IndexMut, Add, Sub, Mul, Div };
use crate::{ Number, Integer, Float };
use std::cmp::PartialOrd;
use std::fmt;

#[derive(Clone)]
pub struct Vector <T: Number> {
    dims : usize,
    vals : Box<[T]>
}

impl <T: Number> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index == 0 { panic!("Index out of bonds") }
        &self.vals[index - 1]
    }
}
impl <T: Number> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index == 0 { panic!("Index out of bonds") }
        &mut self.vals[index - 1]
    }
}

impl <T: Number> Vector<T> {
    pub fn dims(&self) -> usize { self.dims }

    pub fn new(dims: usize, f: impl Fn(usize)->T) -> Vector<T> {
        if dims == 0 { panic!("Invalid vector dimensions on creation") }
        let mut vals = Vec::with_capacity(dims);
        for i in 1..dims+1 {
            vals.push(f(i));
        }
        let vals = vals.into_boxed_slice();
        Vector { dims, vals }
    }
}

impl <T, IE, FE> Vector<T> where
    T: Float<IExp=IE, FExp=FE> + Clone + Add<T, Output=T>,
    IE: Integer + PartialOrd + Into<FE> + Clone,
    FE: Float + Div<FE, Output=FE>
{
    pub fn norm(&self, n: IE) -> T {
        if n <= IE::zero() { panic!("Invalid norm order") }
        let mut sum = T::zero();
        for i in 1..self.dims { sum = sum + self[i].clone().powi(n.clone()) }
        sum.powf(FE::one() / n.into())
    }
}

impl <T, IE, FE> Vector<T> where
    T: Float<IExp=IE, FExp=FE> + Clone + Add<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T>,
    IE: Integer + PartialOrd + Into<FE> + Clone + Add<IE, Output=IE>,
    FE: Float + Div<FE, Output=FE>
{
    pub fn normalize(self) -> Vector<T> {
        let norm = self.norm(IE::one() + IE::one());
        self * (T::one() / norm)
    }
}

impl <T> Vector<T> where T: Number + Clone {
    pub fn repeat(val: T, dims: usize) -> Vector<T> {
        Vector::new(dims, |_| { val.clone() })
    }
}

// SOMA ENTRE VETORES
impl <T, U, V> Add<Vector<U>> for Vector<T> where
    T: Number + Add<U, Output=V> + Clone,
    U: Number + Clone,
    V: Number
{
    type Output = Vector<V>;
    fn add(self, other: Vector<U>) -> Vector<V> {
        if self.dims != other.dims { panic!("Unmatching vectors length") }
        Vector::new(self.dims, |i| { self[i].clone() + other[i].clone() })
    }
}

// DIFERENÇA ENTRE VETORES
impl <T, U, V> Sub<Vector<U>> for Vector<T> where
    T: Number + Sub<U, Output=V> + Clone,
    U: Number + Clone,
    V: Number
{
    type Output = Vector<V>;
    fn sub(self, other: Vector<U>) -> Vector<V> {
        if self.dims != other.dims { panic!("Unmatching vectors length") }
        Vector::new(self.dims, |i| { self[i].clone() - other[i].clone() })
    }
}

// PRODUTO DE VETOR POR ESCALAR
impl <T, U, V> Mul<U> for Vector<T> where
    T: Mul<U, Output=V> + Number + Clone,
    U: Clone,
    V: Number
{
    type Output = Vector<V>;
    fn mul(self, scalar: U) -> Vector<V> {
        Vector::new(self.dims, |i| { self[i].clone() * scalar.clone() })
    }
}

impl <T> fmt::Debug for Vector<T> where
    T: Number + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for i in 1..self.dims+1 {
            buffer.push_str(&format!("{:?}   ", self[i]));
        }
        write!(f, "{}", buffer)
    }
}
