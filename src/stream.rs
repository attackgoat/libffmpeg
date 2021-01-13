use num::BigRational;

use ::{avrational_from_bigrational, bigrational_from_avrational, CodecParams, ff, FFmpegError};

#[derive(Debug, PartialEq)]
pub struct Stream(*mut ff::AVStream);

impl Stream {
    pub fn avg_frame_rate(&self) -> Result<BigRational, FFmpegError> {
        unsafe {
            bigrational_from_avrational(&(*self.0).avg_frame_rate)
        }
    }

    pub fn codec_params(&self) -> Result<CodecParams, FFmpegError> {
        unsafe {
            Ok(CodecParams::from((*self.0).codecpar as *const _))
        }
    }

    pub fn copy_codec_params(&self, codec_params: &CodecParams) -> Result<(), FFmpegError> {
        unsafe {
            match ff::avcodec_parameters_copy((*self.0).codecpar, &**codec_params) {
                e if 0 > e => Err(FFmpegError::from(e)),
                _ => Ok(())
            }
        }
    }

    pub fn index(&self) -> usize {
        unsafe {
            (*self.0).index as usize
        }
    }

    pub fn set_codec_tag(&mut self, value: u32) {
        unsafe {
            (*(*self.0).codec).codec_tag = value;
        }
    }

    pub fn set_time_base(&mut self, value: &BigRational) -> Result<(), FFmpegError> {
        Ok(unsafe {
            (*self.0).time_base = avrational_from_bigrational(value)?;
        })
    }

    pub fn time_base(&self) -> Result<BigRational, FFmpegError> {
        unsafe {
            bigrational_from_avrational(&(*self.0).time_base)
        }
    }
}

impl From<*mut ff::AVStream> for Stream {
    fn from(ptr: *mut ff::AVStream) -> Self {
        Stream(ptr)
    }
}