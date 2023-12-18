use crate::{growable::Growable, mem::Mem};

pub struct BucketQueue<'memory, const BUCKETS: usize, const BUCKET_SIZE: usize, T> {
    buckets: [Growable<'memory, T, BUCKET_SIZE>; BUCKETS],
    last_entered: Option<usize>,
}

impl<'memory, const BUCKETS: usize, const BUCKET_SIZE: usize, T>
    BucketQueue<'memory, BUCKETS, BUCKET_SIZE, T>
{
    pub fn new(mem: &'memory Mem<'memory>) -> Option<Self> {
        let buckets = [(); BUCKETS].map(|_| mem.alloc_growable().unwrap());
        Some(Self {
            buckets,
            last_entered: None,
        })
    }

    /// Return Err if couldn't insert an element
    pub fn insert(&mut self, elem: T, score: usize) -> Result<(), ()> {
        self.buckets
            .get_mut(score)
            .and_then(|e| {
                e.push(elem).and_then(|_| {
                    self.last_entered.as_mut().and_then(|x| {
                        *x = score.min(*x);
                        None::<()>
                    });
                    Some(())
                })
            })
            .ok_or(())
    }

    pub fn pop(&mut self) -> Option<T> {
        self.buckets
            .iter_mut()
            .enumerate()
            .skip(self.last_entered.unwrap_or(0))
            .find_map(|(i, bucket)| {
                if let Some(v) = bucket.pop() {
                    if bucket.len() == 0 {
                        self.last_entered = None;
                    }
                    Some(v)
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod test {
    use crate::mem::Mem;

    use super::BucketQueue;

    #[test]
    fn bucket_queue_works() {
        let mut buffer = [0u8; 1000];
        let mem = Mem::new(&mut buffer);
        let mut buckets: BucketQueue<10, 10, u8> = BucketQueue::new(&mem).unwrap();
        assert_eq!(buckets.pop(), None);
        buckets.insert(5, 3);
        buckets.insert(0, 0);
        buckets.insert(1, 0);
        buckets.insert(10, 1);
        buckets.insert(11, 2);

        assert_eq!(buckets.pop(), Some(1));
        assert_eq!(buckets.pop(), Some(0));
        assert_eq!(buckets.pop(), Some(10));
        assert_eq!(buckets.pop(), Some(11));
        assert_eq!(buckets.pop(), Some(5));
        assert_eq!(buckets.pop(), None);
        buckets.insert(1, 2);
        buckets.insert(2, 2);
        buckets.insert(10, 3);
        assert_eq!(buckets.pop(), Some(2));
        buckets.insert(2, 2);
        assert_eq!(buckets.pop(), Some(2));
        assert_eq!(buckets.pop(), Some(1));
        assert_eq!(buckets.pop(), Some(10));
        assert_eq!(buckets.pop(), None);
    }
}
