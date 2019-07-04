use std::ops::{ Index, IndexMut, Add, Sub, Mul, Div };
use crate::{ Number, Integer, Float };
use std::cmp::PartialOrd;
use std::fmt;

#[derive(Clone, PartialEq)]
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

    pub fn from_vec(vals: Vec<impl Into<T> + Clone>) -> Vector<T> {
        Vector::new(vals.len(), |i| { vals[i-1].clone().into() })
    }
}

impl <T: Number + Clone> Vector<T> {
    pub fn repeat(val: T, dims: usize) -> Vector<T> {
        Vector::new(dims, |_| { val.clone() })
    }

    pub fn scalar_product <U, V> (v1: Vector<T>, v2: Vector<U>) -> V
    where T: Mul<U, Output=V>, U: Number + Clone, V: Number + Add<V, Output=V>
    {
        let mut s = V::zero();
        for i in 1..v1.dims+1 {
            s = s + v1[i].clone() * v2[i].clone();
        }
        s
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
        for i in 1..self.dims+1 { sum = sum + self[i].clone().powi(n.clone()) }
        sum.powf(FE::one() / n.into())
    }

    pub fn normalize(self) -> Vector<T> where
        T: Mul<T, Output=T> + Div<T, Output=T>,
        IE: Add<IE, Output=IE>
    {
        let norm = self.norm(IE::one() + IE::one());
        self * (T::one() / norm)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let v1 = Vector::repeat(2.0, 5);
        let v2 = Vector::from_vec(vec!(2, 2, 2, 2, 2));
        let v3 = Vector::new(5, |_| { 2.0 });
        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
        assert_eq!(v2, v3);
    }

    #[test]
    fn sum() {
        let v1 = Vector::<f64>::from_vec(vec!(1, 2, 3, 4, 5));
        let v2 = Vector::<f64>::from_vec(vec!(0, 1, 2, 4, 0));
        assert_eq!(v1 + v2, Vector::from_vec(vec!(1, 3, 5, 8, 5)));
        let v1 = Vector::<i8>::from_vec(vec!(1, 2, 3, 4, 5));
        let v2 = Vector::<i8>::from_vec(vec!(0, 1, 2, 4, 0));
        let v3 = Vector::repeat(0, 5);
        assert_eq!(v3 + v1 + v2, Vector::from_vec(vec!(1, 3, 5, 8, 5)));
        let v1 = Vector::repeat(0, 5);
        let v2 = Vector::repeat(1, 5);
        assert_eq!(v1 + v2, Vector::new(5, |_| { 1 }))
    }

    #[test]
    fn product() {
        let v1 = Vector::repeat(1, 5);
        assert_eq!(v1*5, Vector::repeat(5, 5));
        let v1 = Vector::<f64>::from_vec(vec!(1, 2, 3, 4, 5));
        assert_eq!(v1*2.0, Vector::from_vec(vec!(2, 4, 6, 8, 10)));
        let v1 = Vector::repeat(1, 5);
        let v2 = Vector::<i32>::from_vec(vec!(1, 2, 3, 4, 5));
        let v3 = Vector::<i32>::from_vec(vec!(2, 1, 2, 1, 2));
        assert_eq!(Vector::scalar_product(v1, v2.clone()), 15);
        assert_eq!(Vector::scalar_product(v2, v3), 24);
    }

    #[test]
    fn norm() {
        let v = Vector::repeat(1.0, 4);
        assert_eq!(v.norm(2), 2.0);
        assert_eq!(v.normalize(), Vector::repeat(0.5, 4));
        let v = Vector::<f64>::from_vec(vec!(2.0, 17.0, 5.3, 1.1));
        assert!((v.norm(2) - 17.9527).abs() < 0.0001);
        {
            let diff = v.normalize() - Vector::<f64>::from_vec(vec!(
                0.111404, 0.946932, 0.29522, 0.0612721
            ));
            for i in 1..5 {
                assert!(diff[i] < 0.00001);
            }
        }
    }
}
