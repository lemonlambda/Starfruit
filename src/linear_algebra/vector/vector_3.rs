use super::VectorType;
use crate::linear_algebra::matrix::matrix_3x3::Matrix3x3;

use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Vector3<T> {
    pub storage: [T; 3],
    pub vector_type: VectorType
}

impl<T: Copy> Vector3<T> {
    pub const fn new(x: T, y: T, z: T, vector_type: VectorType) -> Self {
        Self {
            storage: [x, y, z],
            vector_type
        }
    }
    pub fn transpose(&mut self) {
        self.vector_type = match self.vector_type {
            VectorType::Row => VectorType::Column,
            VectorType::Column => VectorType::Row
        }
    }
}

impl<T: Display> Display for Vector3<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.write_str((match self.vector_type {
            VectorType::Row => format!("[{} {} {}]", self.storage[0], self.storage[1], self.storage[2]),
            VectorType::Column => format!("┌{}┐\n│{}│\n└{}┘", self.storage[0], self.storage[1], self.storage[2])
        }).as_str())
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "nightly")] {
        #[macro_export]
        macro_rules! vec3 {
            ($i1:tt $i2:tt $i3:tt) => {
                #[allow(unused_parens)]
                Vector3::new($i1, $i2, $i3, VectorType::Row)
            }
        }
        pub(crate) use vec3;
    } else {
        #[macro_export]
        macro_rules! vec3 {
            ($i1:tt $i2:tt $i3:tt) => {
                Vector3::new($i1, $i2, $i3, VectorType::Row)
            }
        }
        pub(crate) use vec3;
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<T: Copy + ::std::ops::$op<T, Output = T>> ::std::ops::$op<T> for Vector3<T> {
                    type Output = Vector3<T>;
            
                    fn [<$op:lower>](self, rhs: T) -> Self::Output {
                        Vector3::new(self.storage[0] $operator rhs, self.storage[1] $operator rhs, self.storage[2] $operator rhs, self.vector_type)
                    }
                }
                impl<T: Copy + ::std::ops::$op<Vector3<T>, Output = Vector3<T>>> ::std::ops::$op<Vector3<T>> for Vector3<T> {
                    type Output = Matrix3x3<T>;

                    fn [<$op:lower>](self, rhs: Vector3<T>) -> Self::Output {
                        Matrix3x3::new(self.storage[0] $operator rhs, self.storage[1] $operator rhs, self.storage[2] $operator rhs)
                    }
                }
            )*
        }
    }
}

op_impl!(Add +, Sub -, Mul *, Div /);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn operators() {
        let vec = Vector3::new(10, 20, 30, VectorType::Row);
        assert_eq!(vec + 10, Vector3::new(20, 30, 40, VectorType::Row));
        assert_eq!(vec - 10, Vector3::new(0, 10, 20, VectorType::Row));
        assert_eq!(vec * 10, Vector3::new(100, 200, 300, VectorType::Row));
        assert_eq!(vec / 10, Vector3::new(1, 2, 3, VectorType::Row));
    }

    #[test]
    fn display() {
        let vec = Vector3::new(10, 20, 30, VectorType::Row);
        println!("{}", vec);
        let vec = Vector3::new(10, 20, 30, VectorType::Column);
        println!("{}", vec);
    }
}
