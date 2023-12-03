use core::{
    marker::PhantomData,
    mem::{align_of, size_of, transmute},
    slice,
};

#[derive(Debug, Clone, Copy)]
pub struct Oom;

pub struct Mem<'m> {
    start: *mut u8,
    end: *mut u8,
    alloc_ptr: *mut u8,
    phantom: PhantomData<&'m u8>,
}

impl<'m> Mem<'m> {
    pub fn new(raw: &'m mut [u8]) -> Mem<'m> {
        let start = raw.as_mut_ptr();
        let end = unsafe { start.add(raw.len()) };
        Mem {
            start,
            end,
            alloc_ptr: end,
            phantom: PhantomData,
        }
    }

    unsafe fn bump(&mut self, size: usize, align: usize) -> Result<*mut u8, Oom> {
        debug_assert!(align > 0);
        debug_assert!(align.is_power_of_two());
        let ptr = self.alloc_ptr as usize;
        let new_ptr = ptr.checked_sub(size).ok_or(Oom)?;
        let new_ptr = new_ptr & !(align - 1);
        let start = self.start as usize;
        if new_ptr < start {
            Err(Oom)
        } else {
            let new_ptr = new_ptr as *mut u8;
            self.alloc_ptr = new_ptr;
            Ok(new_ptr)
        }
    }

    pub fn alloc<T>(&mut self, t: T) -> Result<&'m mut T, Oom> {
        let assigned_ptr = unsafe { self.bump(size_of::<T>(), align_of::<T>()) }? as *mut T;
        unsafe { *assigned_ptr = t };
        Ok(unsafe { assigned_ptr.as_mut() }.unwrap())
    }

    pub fn alloc_slice<T, F>(&mut self, length: usize, generator: F) -> Result<&'m mut [T], Oom>
    where
        F: Fn(usize) -> T,
    {
        let size = size_of::<T>() * length;
        let slice_ptr = unsafe { self.bump(size, align_of::<T>()) }? as *mut T;
        (0..length).for_each(|i| {
            unsafe { *slice_ptr.add(i) = generator(i) };
        });
        Ok(unsafe { slice::from_raw_parts_mut(slice_ptr, length) })
    }

    pub fn alloc_slice_default<T: Default>(&mut self, length: usize) -> Result<&'m mut [T], Oom> {
        let size = size_of::<T>() * length;
        let slice_ptr = unsafe { self.bump(size, align_of::<T>()) }? as *mut T;
        (0..length).for_each(|i| {
            unsafe { *slice_ptr.add(i) = T::default() };
        });
        Ok(unsafe { slice::from_raw_parts_mut(slice_ptr, length) })
    }
}

#[cfg(test)]
mod test {
    use core::{
        alloc::{GlobalAlloc, Layout},
        mem::size_of_val,
    };

    use crate::mem::Mem;

    #[test]
    fn mem_works() {
        let mut buffer = [0u8; 1000];
        let mut mem = Mem::new(buffer.as_mut_slice());
        let v1 = mem.alloc(30u8).unwrap();
        let v2 = mem.alloc(100u16).unwrap();
        assert_eq!(*v1, 30);
        assert_eq!(*v2, 100);
    }

    #[test]
    fn alloc_slice_works() {
        let mut buffer = [0u8; 1000];
        let mut mem = Mem::new(buffer.as_mut_slice());
        let v1 = mem.alloc_slice(5, |i| i as u8 + 1 as u8).unwrap();
        let v2 = mem.alloc_slice(5, |i| i as usize + 6 as usize).unwrap();
        assert_eq!(v1, &[1, 2, 3, 4, 5]);
        assert_eq!(v2, &[6, 7, 8, 9, 10]);
        assert_eq!(size_of_val(v1), 5);
        assert_eq!(size_of_val(v2), 40);
    }
}
