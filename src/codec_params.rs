use std::ops::{Deref, DerefMut};

use ::{CodecContext, ff, FFmpegError};

pub struct CodecParams(*mut ff::AVCodecParameters);

impl CodecParams {
    pub fn alloc() -> Self {
        unsafe {
            match ff::avcodec_parameters_alloc() {
                codec_params if !codec_params.is_null() => CodecParams(codec_params),
                _ => panic!("out of memory"),
            }
        }
    }

    pub fn copy_context(&mut self, codec_context: &CodecContext) -> Result<(), FFmpegError> {
        unsafe {
            match ff::avcodec_parameters_from_context(self.0, &**codec_context) {
                e if 0 > e => Err(FFmpegError::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn sample_rate(&self) -> usize {
        unsafe {
            (*self.0).sample_rate as usize
        }
    }

    pub fn set_codec_tag(&mut self, value: usize) {
        unsafe {
            (*self.0).codec_tag = value as u32;
        }
    }
}

impl Deref for CodecParams {
    type Target = ff::AVCodecParameters;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl DerefMut for CodecParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0
        }
    }
}

impl Drop for CodecParams {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ff::avcodec_parameters_free(&mut self.0);
        }
    }
}

//TODO: KILL THIS
impl From<*const ff::AVCodecParameters> for CodecParams {
    fn from(ptr: *const ff::AVCodecParameters) -> Self {
        let mut codec_params = CodecParams::alloc();
        unsafe {
            match ff::avcodec_parameters_copy(&mut *codec_params, ptr) {
                e if 0 > e => panic!("Could not copy codec context"),
                _ => (),
            };
        }
        codec_params
    }
}