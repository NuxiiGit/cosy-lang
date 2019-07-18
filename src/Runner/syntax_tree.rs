/// A recursive enum used to express an abstract syntax tree.
pub enum SExpression<T> {
    Nil,
    Node(T, Vec<SExpression<T>>)
}