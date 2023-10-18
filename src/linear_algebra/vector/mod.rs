pub mod vector_2;
pub mod vector_3;
pub mod vector_4;
pub mod vector_n;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VectorType {
    Row,
    Column
}