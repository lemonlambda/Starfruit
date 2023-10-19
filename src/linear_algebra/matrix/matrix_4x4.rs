use crate::linear_algebra::vector::vector_4::Vector4;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Matrix4x4<T> {
    vectors: [Vector4<T>; 4],
}

impl<T> Matrix4x4<T> {
    pub fn new(v1: Vector4<T>, v2: Vector4<T>, v3: Vector4<T>, v4: Vector4<T>) -> Self {
        assert_eq!(v1.vector_type, v2.vector_type);
        assert_eq!(v2.vector_type, v3.vector_type);
        assert_eq!(v3.vector_type, v4.vector_type);
        Self {
            vectors: [v1, v2, v3, v4],
        }
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<T: Copy + ::std::ops::$op<T, Output = T>> ::std::ops::$op<T> for Matrix4x4<T> {
                    type Output = Matrix4x4<T>;

                    fn [<$op:lower>](self, rhs: T) -> Self::Output {
                        Matrix4x4::new(self.vectors[0] $operator rhs, self.vectors[1] $operator rhs, self.vectors[2] $operator rhs, self.vectors[3] $operator rhs)
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
        macro_rules! matrix4x4 {
            ($i1:tt $i2:tt $i3:tt $i4:tt; $i5:tt $i6:tt $i7:tt $i8:tt; $i9:tt $i10:tt $i11:tt $i12:tt; $i13:tt $i14:tt $i15:tt $i16:tt) => {
                #[allow(unused_parens)]
                Matrix4x4::new(
                    Vector4::new($i1, $i2, $i3, $i4, VectorType::Row),
                    Vector4::new($i5, $i6, $i7, $i8, VectorType::Row),
                    Vector4::new($i9, $i10, $i11, $i12, VectorType::Row),
                    Vector4::new($i13, $i14, $i15, $i16, VectorType::Row)
                )
            }
        }
    } else {
        #[macro_export]
        macro_rules! matrix4x4 {
            ($i1:tt $i2:tt $i3:tt $i4:tt; $i5:tt $i6:tt $i7:tt $i8:tt; $i9:tt $i10:tt $i11:tt $i12:tt; $i13:tt $i14:tt $i15:tt $i16:tt) => {
                Matrix4x4::new(
                    Vector4::new($i1, $i2, $i3, $i4, VectorType::Row),
                    Vector4::new($i5, $i6, $i7, $i8, VectorType::Row),
                    Vector4::new($i9, $i10, $i11, $i12, VectorType::Row),
                    Vector4::new($i13, $i14, $i15, $i16, VectorType::Row)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_algebra::vector::vector_4::vec4;
    use crate::linear_algebra::vector::VectorType;

    #[test]
    fn operators() {
        let matrix = Matrix4x4::new(
            vec4![10 20 30 40],
            vec4![50 60 70 80],
            vec4![90 100 110 120],
            vec4![130 140 150 160],
        );
        assert_eq!(
            matrix + 10,
            Matrix4x4::new(
                vec4![20 30 40 50],
                vec4![60 70 80 90],
                vec4![100 110 120 130],
                vec4![140 150 160 170]
            )
        );
        assert_eq!(
            matrix - 10,
            Matrix4x4::new(
                vec4![0 10 20 30],
                vec4![40 50 60 70],
                vec4![80 90 100 110],
                vec4![120 130 140 150]
            )
        );
        assert_eq!(
            matrix * 10,
            Matrix4x4::new(
                vec4![100 200 300 400],
                vec4![500 600 700 800],
                vec4![900 1000 1100 1200],
                vec4![1300 1400 1500 1600]
            )
        );
        assert_eq!(
            matrix / 10,
            Matrix4x4::new(
                vec4![1 2 3 4],
                vec4![5 6 7 8],
                vec4![9 10 11 12],
                vec4![13 14 15 16]
            )
        );
    }

    #[test]
    fn matrix_macro() {
        matrix4x4!(10 20 30 40; 50 60 70 80; 90 100 110 120; 130 140 150 160);
    }

    #[test]
    #[should_panic]
    fn diff_vector_types() {
        Matrix4x4::new(
            vec4![10 20 30 40],
            Vector4::new(10, 20, 30, 40, VectorType::Column),
            vec4![10 20 30 40],
            vec4![10 20 30 40],
        );
    }
}
