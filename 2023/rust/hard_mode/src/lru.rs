pub enum Pointer {
    Partial(usize),
    Full(usize),
}

pub struct LRU<K, V, const N: usize> {
    raw: [(K, V); N],
    /// points to something
    pointer: Pointer,
}

impl<K, V: Default, const N: usize> LRU<K, V, N>
where
    K: Default + PartialEq,
{
    pub fn new() -> Self {
        Self {
            raw: [(); N].map(|_| (K::default(), V::default())),
            pointer: Pointer::Partial(0),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.pointer {
            Pointer::Partial(p) => self.raw[0..p].iter(),
            Pointer::Full(_) => self.raw[..].iter(),
        }
        .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some((_, v)) = self.raw.iter_mut().find(|(k, _)| *k == key) {
            *v = value;
        } else {
            match self.pointer {
                Pointer::Partial(p) => {
                    self.raw[p] = (key, value);
                    let next = p + 1;
                    if next == N {
                        self.pointer = Pointer::Full(0)
                    } else {
                        self.pointer = Pointer::Partial(next)
                    }
                }
                Pointer::Full(p) => {
                    self.raw[p] = (key, value);
                    self.pointer = Pointer::Full((p + 1).rem_euclid(N))
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::LRU;
    #[test]
    fn lru_works() {
        let mut lru: LRU<usize, usize, 3> = LRU::new();
        lru.put(3, 4);
        lru.put(4, 5);
        assert_eq!(lru.get(&3), Some(&4));
        lru.put(4, 6);
        assert_eq!(lru.get(&4), Some(&6));
        lru.put(5, 1);
        assert_eq!(lru.get(&5), Some(&1));
        lru.put(6, 1);
        assert_eq!(lru.get(&3), None);
        lru.put(6, 2);
        assert_eq!(lru.get(&6), Some(&2));
    }
}
