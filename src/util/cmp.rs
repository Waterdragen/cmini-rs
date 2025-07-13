use std::cmp::Ordering;

#[derive(Debug)]
pub struct Comparator<T: PartialOrd + Copy> {
    pub value: T,
    ordering: Ordering,
}

impl<T: PartialOrd + Copy> Comparator<T> {
    pub fn new(value: T, ordering: Ordering) -> Self {
        Self { value, ordering }
    }
    pub fn check(&self, current: T) -> bool {
        let Some(current_ordering) = current.partial_cmp(&self.value) else {
            return false;
        };
        self.ordering == current_ordering
    }
}