/// A recursive enum used to express an abstract syntax tree.
pub enum SExpression<T> {
    Nil,
    List(T, Vec<SExpression<T>>)
}