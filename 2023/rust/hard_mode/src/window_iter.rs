use core::mem::MaybeUninit;

pub struct WindowIter<I: Iterator, const N: usize> {
    iter: I,
    storage: Option<[I::Item; N]>,
}

impl<I, const N: usize> WindowIter<I, N>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            storage: None,
        }
    }
}

impl<I, const N: usize> Iterator for WindowIter<I, N>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        match self.storage.as_mut() {
            Some(s) => {
                let next_element = self.iter.next()?;
                s.rotate_left(1);
                s[N - 1] = next_element;
                Some(*s)
            }
            None => {
                let mut array: [MaybeUninit<I::Item>; N] = [MaybeUninit::uninit(); N];
                for i in 0..N {
                    array[i] = MaybeUninit::new(self.iter.next()?);
                }
                self.storage = Some(array.map(|i| unsafe { i.assume_init() }));
                self.storage
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use alloc::vec::Vec;

    use super::WindowIter;

    #[test]
    fn window_iter_works() {
        let wi: WindowIter<_, 3> = WindowIter::new([0, 1, 2, 3, 4].into_iter());
        assert_eq!(
            wi.collect::<Vec<_>>(),
            Vec::from([[0, 1, 2], [1, 2, 3], [2, 3, 4]])
        );
        let mut wi: WindowIter<_, 10> = WindowIter::new([0, 1, 2, 3, 4].into_iter());
        assert_eq!(wi.next(), None);
    }
}
