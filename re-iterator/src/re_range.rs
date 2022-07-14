use std::ops::Bound;

pub fn from<T>(start: T) -> ReRangeFrom<T> {
    ReRangeFrom { start }
}

pub struct ReRangeFrom<T> {
    start: T,
}

impl<T> ReRangeFrom<T> {
    pub fn to(self, end: T) -> ReRangeFromTo<T> {
        ReRangeFromTo {
            start: self.start,
            end,
        }
    }
}

pub fn to<T>(end: T) -> ReRangeTo<T> {
    ReRangeTo { end }
}

pub struct ReRangeTo<T> {
    end: T,
}

impl<T> ReRangeTo<T> {
    pub fn from(self, start: T) -> ReRangeFromTo<T> {
        ReRangeFromTo {
            start,
            end: self.end,
        }
    }
}

pub struct ReRangeFromTo<T> {
    start: T,
    end: T,
}
