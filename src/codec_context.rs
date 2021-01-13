use std::ops::{Deref, DerefMut};
use std::ptr::{null, null_mut};

use num::BigRational;

use libc::EAGAIN;

use super::{avrational_from_bigrational, bigrational_from_avrational, Codec, CodecParams, ff, FFmpegError, Frame, Packet};

pub struct CodecContext(*mut ff::AVCodecContext);

impl CodecContext {
    pub fn alloc(codec: &Codec) -> Self {
        unsafe {
            match ff::avcodec_alloc_context3(**codec) {
                codec_context if !codec_context.is_null() => CodecContext(codec_context),
                _ => panic!("out of memory"),
            }
        }
    }

    pub fn bit_rate(&self) -> usize {
        unsafe {
            (*self.0).bit_rate as usize
        }
    }

    pub fn channel_layout(&self) -> u64 {
        unsafe {
            (*self.0).channel_layout
        }
    }

    pub fn channels(&self) -> usize {
        unsafe {
            (*self.0).channels as usize
        }
    }

    pub fn codec(&self) -> Codec {
        unsafe {
            Codec::from((*self.0).codec)
        }
    }

    pub fn color_space(&self) -> ff::AVColorSpace {
        unsafe {
            (*self.0).colorspace
        }
    }

    pub fn copy_params(&mut self, codec_params: &CodecParams) -> Result<(), FFmpegError> {
        unsafe {
            match ff::avcodec_parameters_to_context(self.0, &**codec_params) {
                e if 0 > e => Err(FFmpegError::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub unsafe fn open(&mut self, codec: &Codec) -> Result<(), FFmpegError> {
        match ff::avcodec_open2(self.0, **codec, null_mut()) {
            0 => Ok(()),
            e => Err(FFmpegError::from(e)),
        }
    }

    pub fn flags(&self) -> u32 {
        unsafe {
            (*self.0).flags as u32
        }
    }

    pub fn flush_buffers(&self) {
        unsafe {
            ff::avcodec_flush_buffers(self.0);
        }
    }

    pub fn height(&self) -> usize {
        unsafe {
            (*self.0).height as usize
        }
    }

    pub fn pixel_format(&self) -> ff::AVPixelFormat {
        unsafe {
            (*self.0).pix_fmt
        }
    }

    pub fn receive_frame(&self) -> Result<Option<Frame>, FFmpegError> {
        let mut frame = Frame::alloc();
        unsafe {
            match ff::avcodec_receive_frame(self.0, &mut *frame) {
                0 => Ok(Some(frame)),
                e if e == ff::AVERROR_EOF || e == ff::AVERROR(EAGAIN) => Ok(None),
                e => Err(FFmpegError::from(e)),
            }
        }
    }

    pub fn receive_packet(&self) -> Result<Option<Packet>, FFmpegError> {
        let mut packet = Packet::alloc();
        unsafe {
            match ff::avcodec_receive_packet(self.0, &mut *packet) {
                0 => Ok(Some(packet)),
                e if e == ff::AVERROR_EOF || e == ff::AVERROR(EAGAIN) => Ok(None),
                e => Err(FFmpegError::from(e)),
            }
        }
    }

    pub fn sample_format(&self) -> ff::AVSampleFormat {
        unsafe {
            (*self.0).sample_fmt
        }
    }

    pub fn sample_rate(&self) -> usize {
        unsafe {
            (*self.0).sample_rate as usize
        }
    }

    pub fn send_frame(&self, frame: &Frame) -> Result<(), FFmpegError> {
        unsafe {
            match ff::avcodec_send_frame(self.0, &**frame) {
                0 => Ok(()),
                e => Err(FFmpegError::from(e)),
            }
        }
    }

    pub fn send_null_frame(&self) -> Result<(), FFmpegError> {
        unsafe {
            match ff::avcodec_send_frame(self.0, null()) {
                0 => Ok(()),
                e => Err(FFmpegError::from(e)),
            }
        }
    }

    pub fn send_packet(&self, packet: Packet) -> Result<(), FFmpegError> {
        unsafe {
            match ff::avcodec_send_packet(self.0, &*packet) {
                e if 0 > e => Err(FFmpegError::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn set_bit_rate(&mut self, value: usize) {
        unsafe {
            (*self.0).bit_rate = value as i64;
        }
    }

    pub fn set_channel_layout(&mut self, value: u64) {
        unsafe {
            (*self.0).channel_layout = value;
        }
    }

    pub fn set_channels(&mut self, value: usize) {
        unsafe {
            (*self.0).channels = value as i32;
        }
    }

    pub fn set_color_primaries(&mut self, value: ff::AVColorPrimaries) {
        unsafe {
            (*self.0).color_primaries = value;
        }
    }

    pub fn set_color_range(&mut self, value: ff::AVColorRange) {
        unsafe {
            (*self.0).color_range = value;
        }
    }

    pub fn set_color_space(&mut self, value: ff::AVColorSpace) {
        unsafe {
            (*self.0).colorspace = value;
        }
    }

    pub fn set_color_trc(&mut self, value: ff::AVColorTransferCharacteristic) {
        unsafe {
            (*self.0).color_trc = value;
        }
    }

    pub fn set_flags(&mut self, value: u32) {
        unsafe {
            (*self.0).flags = value as i32;
        }
    }

    pub fn set_framerate(&mut self, value: &BigRational) -> Result<(), FFmpegError> {
        let framerate = avrational_from_bigrational(value)?;
        Ok(unsafe {
            (*self.0).framerate = framerate;
        })
    }

    pub fn set_gop_size(&mut self, value: usize) {
        unsafe {
            (*self.0).gop_size = value as i32;
        }
    }

    pub fn set_height(&mut self, value: usize) {
        unsafe {
            (*self.0).height = value as i32;
        }
    }

    pub fn set_max_b_frames(&mut self, value: usize) {
        unsafe {
            (*self.0).max_b_frames = value as i32;
        }
    }

    pub fn set_pixel_format(&mut self, value: ff::AVPixelFormat) {
        unsafe {
            (*self.0).pix_fmt = value;
        }
    }

    pub fn set_profile(&mut self, value: i32) {
        unsafe {
            (*self.0).profile = value;
        }
    }

    pub fn set_sample_aspect_ratio(&mut self, value: &BigRational) -> Result<(), FFmpegError> {
        Ok(unsafe {
            (*self.0).sample_aspect_ratio = try!(avrational_from_bigrational(value));
        })
    }

    pub fn set_sample_format(&mut self, value: ff::AVSampleFormat) {
        unsafe {
            (*self.0).sample_fmt = value;
        }
    }

    pub fn set_sample_rate(&mut self, value: usize) {
        unsafe {
            (*self.0).sample_rate = value as i32;
        }
    }

    pub fn set_std_compliance(&mut self, value: i32) {
        unsafe {
            (*self.0).strict_std_compliance = value;
        }
    }

    pub fn set_ticks_per_frame(&mut self, value: usize) {
        unsafe {
            (*self.0).ticks_per_frame = value as i32;
        }
    }

    pub fn set_time_base(&mut self, value: &BigRational) -> Result<(), FFmpegError> {
        Ok(unsafe {
            (*self.0).time_base = try!(avrational_from_bigrational(value));
        })
    }

    pub fn set_width(&mut self, value: usize) {
        unsafe {
            (*self.0).width = value as i32;
        }
    }

    pub fn ticks_per_frame(&self) -> usize {
        unsafe {
            (*self.0).ticks_per_frame as usize
        }
    }

    pub fn time_base(&self) -> Result<BigRational, FFmpegError> {
        unsafe {
            bigrational_from_avrational(&(*self.0).time_base)
        }
    }

    pub fn width(&self) -> usize {
        unsafe {
            (*self.0).width as usize
        }
    }
}

impl Deref for CodecContext {
    type Target = ff::AVCodecContext;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl DerefMut for CodecContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0
        }
    }
}

impl Drop for CodecContext {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ff::avcodec_free_context(&mut self.0);
        }
    }
}