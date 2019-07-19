/// A recursive enum used to express an abstract syntax tree.
#[allow(dead_code)]
pub enum SExpression<T> {
    Nil,
    Node(T, Vec<SExpression<T>>)
}