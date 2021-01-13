use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;

use ::{ff, str_from_utf8_cstr_unchecked};

pub fn list() -> Vec<InputFormat> {
    let mut result = vec![];
    let mut ptr = null_mut();
    unsafe {
        loop {
            ptr = ff::av_iformat_next(ptr);
            if ptr.is_null() {
                break;
            } else {
                result.push(InputFormat::from(ptr));
            }
        }
    }
    result
}

pub struct InputFormat(*mut ff::AVInputFormat);

impl InputFormat {
    pub fn name(&self) -> &str {
        unsafe {
            str_from_utf8_cstr_unchecked((*self.0).name)
        }
    }
}

impl From<*mut ff::AVInputFormat> for InputFormat {
    fn from(ptr: *mut ff::AVInputFormat) -> Self {
        InputFormat(ptr)
    }
}

impl Deref for InputFormat {
    type Target = ff::AVInputFormat;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl DerefMut for InputFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0
        }
    }
}
