use crate::linear_algebra::vector::vector_3::Vector3;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Matrix3x3<T> {
    vectors: [Vector3<T>; 3]
}

impl<T> Matrix3x3<T> {
    pub fn new(v1: Vector3<T>, v2: Vector3<T>, v3: Vector3<T>) -> Self {
        assert_eq!(v1.vector_type, v2.vector_type);
        assert_eq!(v2.vector_type, v3.vector_type);
        Self {
            vectors: [v1, v2, v3]
        }
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<T: Copy + ::std::ops::$op<T, Output = T>> ::std::ops::$op<T> for Matrix3x3<T> {
                    type Output = Matrix3x3<T>;
            
                    fn [<$op:lower>](self, rhs: T) -> Self::Output {
                        Matrix3x3::new(self.vectors[0] $operator rhs, self.vectors[1] $operator rhs, self.vectors[2] $operator rhs)
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
        macro_rules! matrix3x3 {
            ($i1:tt $i2:tt $i3:tt; $i4:tt $i5:tt $i6:tt; $i7:tt $i8:tt $i9:tt) => {
                #[allow(unused_parens)]
                Matrix3x3::new(Vector3::new($i1, $i2, $i3, VectorType::Row), Vector3::new($i4, $i5, $i6, VectorType::Row), Vector3::new($i7, $i8, $i9, VectorType::Row))
            }
        }
    } else {
        #[macro_export]
        macro_rules! matrix3x3 {
            ($i1:tt $i2:tt $i3:tt; $i4:tt $i5:tt $i6:tt; $i7:tt $i8:tt $i9:tt) => {
                Matrix3x3::new(Vector3::new($i1, $i2, $i3, VectorType::Row), Vector3::new($i4, $i5, $i6, VectorType::Row), Vector3::new($i7, $i8, $i9, VectorType::Row))
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
        let matrix = Matrix3x3::new(Vector3::new(10, 20, 30, VectorType::Row), Vector3::new(30, 40, 50, VectorType::Row), Vector3::new(60, 70, 80, VectorType::Row));
        assert_eq!(matrix + 10, Matrix3x3::new(Vector3::new(20, 30, 40, VectorType::Row), Vector3::new(40, 50, 60, VectorType::Row), Vector3::new(70, 80, 90, VectorType::Row)));
        assert_eq!(matrix - 10, Matrix3x3::new(Vector3::new(0, 10, 20, VectorType::Row), Vector3::new(20, 30, 40, VectorType::Row), Vector3::new(50, 60, 70, VectorType::Row)));
        assert_eq!(matrix * 10, Matrix3x3::new(Vector3::new(100, 200, 300, VectorType::Row), Vector3::new(300, 400, 500, VectorType::Row), Vector3::new(600, 700, 800, VectorType::Row)));
        assert_eq!(matrix / 10, Matrix3x3::new(Vector3::new(1, 2, 3, VectorType::Row), Vector3::new(3, 4, 5, VectorType::Row), Vector3::new(6, 7, 8, VectorType::Row)));
    }

    #[test]
    fn matrix_macro() {
        assert_eq!(matrix3x3!(10 20 30; 40 50 60; 70 80 90), Matrix3x3::new(Vector3::new(10, 20, 30, VectorType::Row), Vector3::new(40, 50, 60, VectorType::Row), Vector3::new(70, 80, 90, VectorType::Row)));
    }

    #[test]
    #[should_panic]
    fn diff_vector_types() {
        Matrix3x3::new(Vector3::new(10, 20, 30, VectorType::Row), Vector3::new(10, 20, 30, VectorType::Column), Vector3::new(10, 20, 30, VectorType::Row));
    }
}