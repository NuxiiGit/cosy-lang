/// A struct which provides a way of checking whether a value satisfies one or more conditions.
pub struct Comparator<T : PartialEq> {
    value : T,
    satisfied : bool
}
impl<T : PartialEq> Comparator<T> {
    /// Creates a new comparator from this value.
    pub fn from(value : T) -> Self {
        Self {
            value,
            satisfied : false
        }
    }

    /// Satisfies the value if it is equal to the target value.
    pub fn equals(mut self, value : T) -> Self {
        if !self.satisfied {
            self.satisfied = self.value == value;
        }
        self
    }

    /// Satisfies the value if it holds for some predicate.
    pub fn satisfies(mut self, p : fn(&T) -> bool) -> Self {
        if !self.satisfied {
            self.satisfied = p(&self.value);
        }
        self
    }

    /// Consumes the comparator and returns whether the value was satisfied.
    pub fn check(self) -> bool {
        self.satisfied
    }
}