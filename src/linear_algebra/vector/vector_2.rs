use crate::linear_algebra::matrix::matrix_2x2::Matrix2x2;

use super::VectorType;

use std::fmt::{Display, Error, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Vector2<T> {
    pub storage: [T; 2],
    pub vector_type: VectorType,
}

impl<T: Copy> Vector2<T> {
    pub const fn new(x: T, y: T, vector_type: VectorType) -> Self {
        Self {
            storage: [x, y],
            vector_type,
        }
    }
    pub fn transpose(&mut self) {
        self.vector_type = match self.vector_type {
            VectorType::Row => VectorType::Column,
            VectorType::Column => VectorType::Row,
        }
    }
}

impl<T: Display> Display for Vector2<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.write_str(
            (match self.vector_type {
                VectorType::Row => format!("[{} {}]", self.storage[0], self.storage[1]),
                VectorType::Column => format!("┌{}┐\n└{}┘", self.storage[0], self.storage[1]),
            })
            .as_str(),
        )
    }
}

macro_rules! vec2 {
    ($i1:tt $i2:tt) => {
        Vector2::new($i1, $i2, VectoryType::Row)
    };
}
cfg_if::cfg_if! {
    if #[cfg(feature = "nightly")] {
        #[macro_export]
        macro_rules! vec2 {
            ($i1:tt $i2:tt) => {
                #[allow(unused_parens)]
                Vector2::new($i1, $i2, VectorType::Row)
            }
        }
        pub(crate) use vec2;
    } else {
        #[macro_export]
        macro_rules! vec2 {
            ($i1:tt $i2:tt) => {
                Vector2::new($i1, $i2, VectorType::Row)
            }
        }
        pub(crate) use vec2;
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<T: Copy + ::std::ops::$op<T, Output = T>> ::std::ops::$op<T> for Vector2<T> {
                    type Output = Vector2<T>;

                    fn [<$op:lower>](self, rhs: T) -> Self::Output {
                        Vector2::new(self.storage[0] $operator rhs, self.storage[1] $operator rhs, self.vector_type)
                    }
                }
                impl<T: Copy + ::std::ops::$op<Vector2<T>, Output = Vector2<T>>> ::std::ops::$op<Vector2<T>> for Vector2<T> {
                    type Output = Matrix2x2<T>;

                    fn [<$op:lower>](self, rhs: Vector2<T>) -> Self::Output {
                        Matrix2x2::new(self.storage[0] $operator rhs, self.storage[1] $operator rhs)
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
        let vec = Vector2::new(10, 20, VectorType::Row);
        assert_eq!(vec + 10, Vector2::new(20, 30, VectorType::Row));
        assert_eq!(vec - 10, Vector2::new(0, 10, VectorType::Row));
        assert_eq!(vec * 10, Vector2::new(100, 200, VectorType::Row));
        assert_eq!(vec / 10, Vector2::new(1, 2, VectorType::Row));
    }

    #[test]
    fn display() {
        let vec = Vector2::new(10, 20, VectorType::Row);
        println!("{}", vec);
        let vec = Vector2::new(10, 20, VectorType::Column);
        println!("{}", vec);
    }
}
