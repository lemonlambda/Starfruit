use crate::linear_algebra::vector::vector_2::Vector2;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Matrix2x2<T> {
    vectors: [Vector2<T>; 2]
}

impl<T> Matrix2x2<T> {
    pub fn new(v1: Vector2<T>, v2: Vector2<T>) -> Self {
        assert_eq!(v1.vector_type, v2.vector_type);
        Self {
            vectors: [v1, v2]
        }
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<U: Copy, T: Copy + ::std::ops::$op<U, Output = T>> ::std::ops::$op<U> for Matrix2x2<T> {
                    type Output = Matrix2x2<T>;
            
                    fn [<$op:lower>](self, rhs: U) -> Self::Output {
                        Matrix2x2::new(self.vectors[0] $operator rhs, self.vectors[1] $operator rhs)
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
    use crate::linear_algebra::vector::VectorType;

    #[test]
    fn operators() {
        let matrix = Matrix2x2::new(Vector2::new(10, 20, VectorType::Row), Vector2::new(30, 40, VectorType::Row));
        assert_eq!(matrix + 10, Matrix2x2::new(Vector2::new(20, 30, VectorType::Row), Vector2::new(40, 50, VectorType::Row)));
        assert_eq!(matrix - 10, Matrix2x2::new(Vector2::new(0, 10, VectorType::Row), Vector2::new(20, 30, VectorType::Row)));
        assert_eq!(matrix * 10, Matrix2x2::new(Vector2::new(100, 200, VectorType::Row), Vector2::new(300, 400, VectorType::Row)));
        assert_eq!(matrix / 10, Matrix2x2::new(Vector2::new(1, 2, VectorType::Row), Vector2::new(3, 4, VectorType::Row)));
    }

    #[test]
    #[should_panic]
    fn diff_vector_types() {
        Matrix2x2::new(Vector2::new(10, 20, VectorType::Row), Vector2::new(10, 20, VectorType::Column));
    }
}