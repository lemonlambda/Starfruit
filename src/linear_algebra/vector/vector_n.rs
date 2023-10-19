use super::VectorType;

use std::fmt::{Display, Formatter, Error};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct VectorN<T> {
    storage: Vec<T>,
    vector_type: VectorType
}

impl<T> VectorN<T> {
    pub fn new(vec: Vec<T>, vector_type: VectorType) -> Self {
        if vec.len() < 5 {
            panic!("Do not use `VectorN` for Vectors below 5 items");
        }

        Self {
            storage: vec,
            vector_type
        }
    }
    pub fn transpose(&mut self) {
        self.vector_type = match self.vector_type {
            VectorType::Row => VectorType::Column,
            VectorType::Column => VectorType::Row
        };
    }
}

impl<T: Display> Display for VectorN<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.write_str((match self.vector_type {
            VectorType::Row => format!("[{}{}]", self.storage[0..self.storage.len() - 1].iter().map(|n| format!("{} ", n)).collect::<String>(), self.storage[self.storage.len() - 1]),
            VectorType::Column => format!("┌{}┐\n{}└{}┘", self.storage[0], self.storage[1..(self.storage.len() - 1)].iter().map(|n| format!("│{}│\n", n)).collect::<String>(), self.storage[self.storage.len() - 1])
        }).as_str())
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<T: Copy + ::std::ops::$op<T, Output = T>> ::std::ops::$op<T> for VectorN<T> {
                    type Output = VectorN<T>;
            
                    fn [<$op:lower>](self, rhs: T) -> Self::Output {
                        VectorN::new(self.storage.iter().map(|x| *x $operator rhs).collect::<Vec<_>>(), self.vector_type)
                    }
                }
                // impl<T: Copy + ::std::ops::$op<VectorN<T>, Output = VectorN<T>>> ::std::ops::$op<VectorN<T>> for VectorN<T> {
                //     type Output = MatrixNxN<T>;

                //     fn [<$op:lower>](self, rhs: VectorN<T>) -> Self::Output {
                //         MatrixNxN::new(self.storage[0] $operator rhs, self.storage[1] $operator rhs)
                //     }
                // }
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
        let vec = VectorN::new(vec![10, 20, 30, 40, 50], VectorType::Row);
        assert_eq!(vec.clone() + 10, VectorN::new(vec![20, 30, 40, 50, 60], VectorType::Row));
        assert_eq!(vec.clone() - 10, VectorN::new(vec![0, 10, 20, 30, 40], VectorType::Row));
        assert_eq!(vec.clone() * 10, VectorN::new(vec![100, 200, 300, 400, 500], VectorType::Row));
        assert_eq!(vec / 10, VectorN::new(vec![1, 2, 3, 4, 5], VectorType::Row));
    }

    #[test]
    #[should_panic]
    fn less_than_5() {
        VectorN::new(vec![0], VectorType::Row);
    }

    #[test]
    fn display() {
        let vec = VectorN::new(vec![10, 20, 30, 40, 50], VectorType::Row);
        println!("{vec}");
        let vec = VectorN::new(vec![10, 20, 30, 40, 50], VectorType::Column);
        println!("{vec}");
    }
}
