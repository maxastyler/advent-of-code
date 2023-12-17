use std::{cmp::Reverse, collections::BinaryHeap};

struct MinHeapWrapper<T> {
    inner: T,
    key: Reverse<usize>,
}

impl<T> Eq for MinHeapWrapper<T> {}

impl<T> PartialEq for MinHeapWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<T> PartialOrd for MinHeapWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<T> Ord for MinHeapWrapper<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

pub struct MinHeap<T> {
    inner: BinaryHeap<MinHeapWrapper<T>>,
}

impl<T> MinHeap<T> {
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, value: T, key: usize) {
        self.inner.push(MinHeapWrapper {
            inner: value,
            key: Reverse(key),
        })
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop().map(|x| x.inner)
    }
}

#[cfg(test)]
mod test {
    use super::MinHeap;

    #[test]
    fn min_heap_test() {
        let mut heap = MinHeap::new();
        heap.insert(3, 2);
        heap.insert(34, 3);
        heap.insert(-10, 10);
        heap.insert(-3, 0);
        assert_eq!(heap.pop(), Some(-3));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(34));
        assert_eq!(heap.pop(), Some(-10));
        assert_eq!(heap.pop(), None)
    }
}
