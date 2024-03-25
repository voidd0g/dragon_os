use core::mem::{size_of, swap};

pub struct Queue<T> {
    buf: *mut Option<T>,
    capacity: usize,
    count: usize,
    out_pos: usize,
    in_pos: usize,
}

impl<T> Queue<T> {
    pub const fn new(buf: *mut Option<T>, capacity: usize) -> Self {
        Self {
            buf,
            capacity,
            count: 0,
            out_pos: 0,
            in_pos: 0,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let target = unsafe {
            (((self.buf as usize) + self.out_pos * size_of::<Option<T>>()) as *mut Option<T>)
                .as_mut()
        }
        .unwrap();
        match target {
            Some(_) => {
                let mut prev_val = None;
                swap(&mut prev_val, target);
                self.out_pos = (self.out_pos + 1) % self.capacity;
                self.count -= 1;
                prev_val
            }
            None => None,
        }
    }

    pub fn front(&self) -> &Option<T> {
        unsafe {
            (((self.buf as usize) + self.out_pos * size_of::<Option<T>>()) as *mut Option<T>)
                .as_ref()
        }
        .unwrap()
    }

    pub fn push(&mut self, v: T) -> Result<(), ()> {
        let target = unsafe {
            (((self.buf as usize) + self.in_pos * size_of::<Option<T>>()) as *mut Option<T>)
                .as_mut()
        }
        .unwrap();
        match target {
            Some(_) => Err(()),
            None => {
                *target = Some(v);
                self.in_pos = (self.in_pos + 1) % self.capacity;
                self.count += 1;
                Ok(())
            }
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
