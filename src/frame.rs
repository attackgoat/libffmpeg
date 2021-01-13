use std::cmp::max;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};
use std::slice::from_raw_parts_mut;

use ::{ff, FFmpegError};

pub struct Frame(*mut ff::AVFrame);

impl Frame {
    pub fn alloc() -> Self {
        unsafe {
            match ff::av_frame_alloc() {
                frame if !frame.is_null() => Frame(frame),
                _ => panic!("out of memory"),
            }
        }
    }

    pub fn best_effort_timestamp(&self) -> usize {
        unsafe {
            (*self.0).best_effort_timestamp as usize
        }
    }

    pub fn channels(&self) -> usize {
        unsafe {
            (*self.0).channels as usize
        }
    }

    pub fn channel_layout(&self) -> u64 {
        unsafe {
            (*self.0).channel_layout
        }
    }

    pub fn color_space(&self) -> ff::AVColorSpace {
        unsafe {
            (*self.0).colorspace
        }
    }

    pub fn data<T>(&self, index: usize) -> &[T] {
        self.data_mut(index)
    }

    pub fn data_mut<T>(&self, index: usize) -> &mut [T] {
        let len = max(self.samples(), self.height() * self.line_size(index));

        unsafe {
            from_raw_parts_mut(transmute((*self.0).data[index]), len)
        }
    }

    pub fn duration(&self) -> Option<usize> {
        unsafe {
            let duration = (*self.0).pkt_duration;
            if 0 < duration {
                Some(duration as usize)
            } else {
                None
            }
        }
    }

    pub fn get_buffer(&mut self) -> Result<(), FFmpegError> {
        unsafe {
            match ff::av_frame_get_buffer(self.0, 32) {
                0 => Ok(()),
                e => Err(FFmpegError::from(e)),
            }
        }
    }

    pub fn height(&self) -> usize {
        unsafe {
            (*self.0).height as usize
        }
    }

    pub fn line_size(&self, index: usize) -> usize {
        unsafe {
            (*self.0).linesize[index] as usize
        }
    }

    pub fn make_writable(&mut self) -> Result<(), FFmpegError> {
        unsafe {
            match ff::av_frame_make_writable(self.0) {
                0 => Ok(()),
                e => Err(FFmpegError::from(e)),
            }
        }
    }

    pub fn pixel_format(&self) -> ff::AVPixelFormat {
        unsafe {
            transmute((*self.0).format)
        }
    }

    pub fn pts(&self) -> Option<usize> {
        unsafe {
            let pts = (*self.0).pts;
            if 0 > pts {
                None
            } else {
                Some(pts as usize)
            }
        }
    }

    pub fn samples(&self) -> usize {
        unsafe {
            (*self.0).nb_samples as usize
        }
    }

    pub fn sample_format(&self) -> ff::AVSampleFormat {
        unsafe {
            transmute((*self.0).format)
        }
    }

    pub fn sample_rate(&self) -> usize {
        unsafe {
            ff::av_frame_get_sample_rate(self.0) as usize
        }
    }

    pub fn set_channels(&mut self, value: usize) {
        unsafe {
            (*self.0).channels = value as i32;
        }
    }

    pub fn set_channel_layout(&mut self, value: u64) {
        unsafe {
            (*self.0).channel_layout = value;
        }
    }

    pub fn set_color_space(&self, value: ff::AVColorSpace) {
        unsafe {
            (*self.0).colorspace = value;
        }
    }

    pub fn set_height(&self, value: usize) {
        unsafe {
            (*self.0).height = value as i32;
        }
    }

    pub fn set_pixel_format(&self, value: ff::AVPixelFormat) {
        unsafe {
            (*self.0).format = value as i32;
        }
    }

    pub fn set_pts(&mut self, value: usize) {
        unsafe {
            (*self.0).pts = value as i64;
        }
    }

    pub fn set_sample_format(&self, value: ff::AVSampleFormat) {
        unsafe {
            (*self.0).format = value as i32;
        }
    }

    pub fn set_sample_rate(&mut self, value: usize) {
        unsafe {
            ff::av_frame_set_sample_rate(self.0, value as i32);
        }
    }

    pub fn set_samples(&mut self, value: usize) {
        unsafe {
            (*self.0).nb_samples = value as i32;
        }
    }

    pub fn set_width(&self, value: usize) {
        unsafe {
            (*self.0).width = value as i32;
        }
    }

    pub fn width(&self) -> usize {
        unsafe {
            (*self.0).width as usize
        }
    }
}

impl Deref for Frame {
    type Target = ff::AVFrame;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl DerefMut for Frame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0
        }
    }
}

impl Drop for Frame {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ff::av_frame_free(&mut self.0);
        }
    }
}