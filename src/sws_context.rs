use std::ptr::{null, null_mut};
//use std::slice::from_raw_parts;

use ::{ff, FFmpegError, Frame};

pub struct SwsContext(*mut ff::SwsContext);

impl SwsContext {
    pub unsafe fn new(src_width: usize, src_height: usize, src_format: ff::AVPixelFormat, dst_width: usize, dst_height: usize, dst_format: ff::AVPixelFormat) -> Result<Self, FFmpegError> {
        // Sanity check
        if 0 == ff::sws_isSupportedInput(src_format) {
            return Err(FFmpegError::DecoderNotFound);
        }

        // Sanity check
        if 0 == ff::sws_isSupportedOutput(dst_format) {
            return Err(FFmpegError::EncoderNotFound);
        }

        // Create a new context that goes from src -> dst using bicubic scaling and default filters
        let sw = src_width as i32;
        let sh = src_height as i32;
        let dw = dst_width as i32;
        let dh = dst_height as i32;
        let flags = ff::SWS_BICUBIC;
        let context = ff::sws_getContext(sw, sh, src_format, dw, dh, dst_format, flags, null_mut(), null_mut(), null());

        // Sanity check
        if context.is_null() {
            return Err(FFmpegError::FilterNotFound);
        }

        Ok(SwsContext(context))
    }

    pub unsafe fn is_src_range_mpeg(&self) -> Result<bool, FFmpegError> {
        let mut inv_table = null_mut();
        let mut src_range: i32 = 0;
        let mut table = null_mut();
        let mut dst_range: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut s: i32 = 0;
        match ff::sws_getColorspaceDetails(self.0, &mut inv_table, &mut src_range, &mut table, &mut dst_range, &mut b, &mut c, &mut s) {
            -1 => Err(FFmpegError::InvalidData),
            _ => Ok(0 == src_range)
        }
    }

    /*pub unsafe fn is_dst_range_mpeg(&self) -> bool {
        let mut inv_table = null_mut();
        let mut table = null_mut();
        let mut src_range: i32 = 9;
        let mut dst_range: i32 = 10;
        let mut b: i32 = 11;
        let mut c: i32 = 12;
        let mut s: i32 = 13;
        let z = ff::sws_getColorspaceDetails(self.0,
                                    &mut inv_table,
                                    &mut src_range,
                                    &mut table,
                                    &mut dst_range,
                                    &mut b,
                                    &mut c,
                                    &mut s);

        let inv = from_raw_parts(inv_table, 4);
        let tbl = from_raw_parts(table, 4);

        0 == dst_range
    }*/

    // TODO: This doesn't work not sure why
    /*pub unsafe fn set_src_range_mpeg(&mut self, value: bool) -> Result<(), FFmpegError> {
        let mut inv_table = null_mut();
        let mut src_range: i32 = 0;
        let mut table = null_mut();
        let mut dst_range: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut s: i32 = 0;
        match ff::sws_getColorspaceDetails(self.0, &mut inv_table, &mut src_range, &mut table, &mut dst_range, &mut b, &mut c, &mut s) {
            -1 => return Err(FFmpegError::InvalidData),
            _ => (),
        }

        let src_range = if value {
            0
        } else {
            1
        };

        match ff::sws_setColorspaceDetails(self.0, inv_table, src_range, table, dst_range, b, c, s) {
            -1 => Err(FFmpegError::InvalidData),
            _ => Ok(()),
        }
    }*/

    pub unsafe fn scale(&self, src: &Frame, dst: &Frame) -> Result<(), FFmpegError> {
        let sd = (*src).data.as_ptr() as *const _;
        let sl = (*src).linesize.as_ptr();
        let sh = src.height() as i32;
        let dd = (*dst).data.as_ptr() as *const _;
        let dl = (*dst).linesize.as_ptr();
        let slice_height = ff::sws_scale(self.0, sd, sl, 0, sh, dd, dl);

        if slice_height == dst.height() as i32 {
            Ok(())
        } else {
            Err(FFmpegError::InvalidData)
        }
    }
}

impl Drop for SwsContext {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ff::sws_freeContext(self.0);
        }
    }
}