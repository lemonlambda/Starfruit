use super::VectorType;

use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Vector2<T> {
    pub storage: [T; 2],
    pub vector_type: VectorType
}

impl<T: Copy> Vector2<T> {
    pub const fn new(x: T, y: T, vector_type: VectorType) -> Self {
        Self {
            storage: [x, y],
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

impl<T: Display> Display for Vector2<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.write_str((match self.vector_type {
            VectorType::Row => format!("[{} {}]", self.storage[0], self.storage[1]),
            VectorType::Column => format!("┌{}┐\n└{}┘", self.storage[0], self.storage[1])
        }).as_str())
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<U: Copy, T: Copy + ::std::ops::$op<U, Output = T>> ::std::ops::$op<U> for Vector2<T> {
                    type Output = Vector2<T>;
            
                    fn [<$op:lower>](self, rhs: U) -> Self::Output {
                        Vector2::new(self.storage[0] $operator rhs, self.storage[1] $operator rhs, self.vector_type)
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