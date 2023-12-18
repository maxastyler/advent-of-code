pub struct Growable<T, const Limit: usize> {
    start: *mut T,
    length: usize,
}

impl<T, const Limit: usize> Growable<T, Limit> {
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self {
            start: ptr,
            length: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        (0..self.length).map(|i| unsafe { (self.start.add(i)).as_ref().unwrap() })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        (0..self.length).map(|i| unsafe { (self.start.add(i)).as_mut().unwrap() })
    }

    pub fn push(&mut self, value: T) -> Option<&mut T> {
        if self.length < Limit {
            let pos = unsafe { self.start.add(self.length).as_mut() }.unwrap();
            *pos = value;
            self.length += 1;
            Some(pos)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            None
        } else {
            Some(unsafe { self.start.add(index).as_mut() }.unwrap())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
	    let ret_val = unsafe { self.start.add(self.length-1).read() };
	    self.length -= 1;
            Some(ret_val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mem::Mem;

    #[test]
    fn new_works() {
        let mut buffer = [0u8; 10000];
        Mem::new(&mut buffer);
    }
}
