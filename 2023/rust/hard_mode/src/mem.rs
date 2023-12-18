use core::cell::Cell;
use core::{
    marker::PhantomData,
    mem::{align_of, size_of},
    slice,
};

use crate::growable::Growable;

#[derive(Debug, Clone, Copy)]
pub struct Oom;

pub struct Mem<'m> {
    start: *mut u8,
    alloc_ptr: Cell<*mut u8>,
    phantom: PhantomData<&'m u8>,
}

impl<'m> Mem<'m> {
    pub fn new(raw: &'m mut [u8]) -> Mem<'m> {
        let start = raw.as_mut_ptr();
        let end = unsafe { start.add(raw.len()) };
        Mem {
            start,
            alloc_ptr: Cell::new(end),
            phantom: PhantomData,
        }
    }

    unsafe fn bump(&self, size: usize, align: usize) -> Result<*mut u8, Oom> {
        debug_assert!(align > 0);
        debug_assert!(align.is_power_of_two());
        let ptr = self.alloc_ptr.get() as usize;
        let new_ptr = ptr.checked_sub(size).ok_or(Oom)?;
        let new_ptr = new_ptr & !(align - 1);
        let start = self.start as usize;
        if new_ptr < start {
            Err(Oom)
        } else {
            let new_ptr = new_ptr as *mut u8;
            self.alloc_ptr.set(new_ptr);
            Ok(new_ptr)
        }
    }

    pub fn alloc<T>(&self, t: T) -> Result<&'m mut T, Oom> {
        let assigned_ptr = unsafe { self.bump(size_of::<T>(), align_of::<T>()) }? as *mut T;
        unsafe { *assigned_ptr = t };
        Ok(unsafe { assigned_ptr.as_mut() }.unwrap())
    }

    pub fn alloc_slice<T, F>(&self, length: usize, mut generator: F) -> Result<&'m mut [T], Oom>
    where
        F: FnMut(usize) -> T,
    {
        let size = size_of::<T>() * length;
        let slice_ptr = unsafe { self.bump(size, align_of::<T>()) }? as *mut T;
        (0..length).for_each(|i| {
            unsafe { *slice_ptr.add(i) = generator(i) };
        });
        Ok(unsafe { slice::from_raw_parts_mut(slice_ptr, length) })
    }

    pub fn alloc_slice_default<T: Default>(&self, length: usize) -> Result<&'m mut [T], Oom> {
        let size = size_of::<T>() * length;
        let slice_ptr = unsafe { self.bump(size, align_of::<T>()) }? as *mut T;
        (0..length).for_each(|i| {
            unsafe { *slice_ptr.add(i) = T::default() };
        });
        Ok(unsafe { slice::from_raw_parts_mut(slice_ptr, length) })
    }

    /// Allocate a slice from an iterator, where the length of the iterator is known beforehand
    pub fn alloc_slice_from_iter<T>(
        &self,
        length: usize,
        mut iter: impl Iterator<Item = T>,
    ) -> Result<&'m mut [T], Oom> {
        let size = size_of::<T>() * length;
        let slice_ptr = unsafe { self.bump(size, align_of::<T>()) }? as *mut T;
        (0..length).for_each(|i| unsafe { *slice_ptr.add(i) = iter.next().unwrap() });
        Ok(unsafe { slice::from_raw_parts_mut(slice_ptr, length) })
    }

    pub fn alloc_growable<T, const Limit: usize>(&self) -> Result<Growable<T, Limit>, Oom> {
        let size = size_of::<T>() * Limit;
        let growable_ptr = unsafe { self.bump(size, align_of::<T>()) }? as *mut T;
        Ok(unsafe { Growable::new(growable_ptr) })
    }

    pub fn clone_slice<T>(&self, slice: &[T]) -> Result<&mut [T], Oom> {
        let size = slice.len() * size_of::<T>();
        let new_ptr = unsafe { self.bump(size, align_of::<T>()) }? as *mut T;
        unsafe { core::ptr::copy(slice.as_ptr(), new_ptr, slice.len()) };
        Ok(unsafe { slice::from_raw_parts_mut(new_ptr, slice.len()) })
    }
}

#[cfg(test)]
mod test {
    use core::mem::size_of_val;

    use crate::mem::Mem;

    #[test]
    fn mem_works() {
        let mut buffer = [0u8; 1000];
        let mem = Mem::new(buffer.as_mut_slice());
        let v1 = mem.alloc(30u8).unwrap();
        let v2 = mem.alloc(100u16).unwrap();
        assert_eq!(*v1, 30);
        assert_eq!(*v2, 100);
    }

    #[test]
    fn alloc_slice_works() {
        let mut buffer = [0u8; 1000];
        let mem = Mem::new(buffer.as_mut_slice());
        let v1 = mem.alloc_slice(5, |i| i as u8 + 1 as u8).unwrap();
        let v2 = mem.alloc_slice(5, |i| i as usize + 6 as usize).unwrap();
        assert_eq!(v1, &[1, 2, 3, 4, 5]);
        assert_eq!(v2, &[6, 7, 8, 9, 10]);
        assert_eq!(size_of_val(v1), 5);
        assert_eq!(size_of_val(v2), 40);
    }
}
