use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;

use ::{ff, str_from_utf8_cstr_unchecked};

pub fn list() -> Vec<OutputFormat> {
    let mut result = vec![];
    let mut ptr = null_mut();
    unsafe {
        loop {
            ptr = ff::av_oformat_next(ptr);
            if ptr.is_null() {
                break;
            } else {
                result.push(OutputFormat::from(ptr));
            }
        }
    }
    result
}

pub struct OutputFormat(*mut ff::AVOutputFormat);

impl OutputFormat {
    pub fn name(&self) -> &str {
        unsafe {
            str_from_utf8_cstr_unchecked((*self.0).name)
        }
    }
}

impl From<*mut ff::AVOutputFormat> for OutputFormat {
    fn from(ptr: *mut ff::AVOutputFormat) -> Self {
        OutputFormat(ptr)
    }
}

impl Deref for OutputFormat {
    type Target = ff::AVOutputFormat;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl DerefMut for OutputFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0
        }
    }
}
