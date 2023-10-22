use crate::linear_algebra::vector::vector_2::Vector2;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MatrixNxN<T> {
    vectors: Vec<VectorN<T>>>
}

impl<T> Matrix2x2<T> {
    pub fn new(rows: Vec<VectorN<T>>, v2: Vector2<T>) -> Self {
        rows.iter().flat_map(|x| assert_eq!(x.vector_type))
        Self {
            vectors: [v1, v2]
        }
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<T: Copy + ::std::ops::$op<T, Output = T>> ::std::ops::$op<T> for Matrix2x2<T> {
                    type Output = Matrix2x2<T>;
            
                    fn [<$op:lower>](self, rhs: T) -> Self::Output {
                        Matrix2x2::new(self.vectors[0] $operator rhs, self.vectors[1] $operator rhs)
                    }
                }
            )*
        }
    }
}

op_impl!(Add +, Sub -, Mul *, Div /);


cfg_if::cfg_if! {
    if #[cfg(feature = "nightly")] {
        #[macro_export]
        macro_rules! matrix2x2 {
            ($i1:tt $i2:tt; $i3:tt $i4:tt) => {
                #[allow(unused_parens)]
                Matrix2x2::new(Vector2::new($i1, $i2, VectorType::Row), Vector2::new($i3, $i4, VectorType::Row))
            }
        }
    } else {
        #[macro_export]
        macro_rules! matrix2x2 {
            ($i1:tt $i2:tt; $i3:tt $i4:tt) => {
                Matrix2x2::new(Vector2::new($i1, $i2, VectorType::Row), Vector2::new($i3, $i4, VectorType::Row))
            }
        }
    }
}


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
    fn matrix_macro() {
        assert_eq!(matrix2x2!((20 - 10) 20; 30 40), Matrix2x2::new(Vector2::new(10, 20, VectorType::Row), Vector2::new(30, 40, VectorType::Row)));
    }

    #[test]
    #[should_panic]
    fn diff_vector_types() {
        Matrix2x2::new(Vector2::new(10, 20, VectorType::Row), Vector2::new(10, 20, VectorType::Column));
    }
}