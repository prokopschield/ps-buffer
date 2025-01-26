use crate::Buffer;

impl Ord for Buffer {
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        if max[..] > self[..] {
            max
        } else if min[..] < self[..] {
            min
        } else {
            self
        }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self[..].cmp(&other[..])
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self[..] > other[..] {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self[..] < other[..] {
            self
        } else {
            other
        }
    }
}
