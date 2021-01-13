#![deny(warnings)]

pub extern crate ffmpeg_sys as ff;
extern crate libc;
pub extern crate num;

mod codec;
mod codec_context;
mod codec_params;
mod format_context;
mod frame;
mod input_format;
mod output_format;
mod packet;
mod stream;
mod sws_context;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::str::from_utf8_unchecked;

use num::{BigInt, BigRational, FromPrimitive, ToPrimitive};

pub use self::codec::Codec;
pub use self::codec_context::CodecContext;
pub use self::codec_params::CodecParams;
pub use self::format_context::FormatContext;
pub use self::frame::Frame;
pub use self::input_format::{InputFormat, list as input_format_list};
pub use self::output_format::{OutputFormat, list as output_format_list};
pub use self::packet::Packet;
pub use self::stream::Stream;
pub use self::sws_context::SwsContext;

use self::FFmpegError::*;

pub fn avrational_from_bigrational(value: &BigRational) -> Result<ff::AVRational, FFmpegError> {
    let reduced = reduce_bigrational_to_i32(value);
    let numer = reduced.numer().to_i32();
    let denom = reduced.denom().to_i32();

    if numer.is_none() || denom.is_none() {
        Err(FFmpegError::InvalidData)
    } else {
        Ok(ff::AVRational {
            num: numer.unwrap(),
            den: denom.unwrap(),
        })
    }
}

pub fn bigrational_from_avrational(value: &ff::AVRational) -> Result<BigRational, FFmpegError> {
    let numer = BigInt::from_i32(value.num);
    let denom = BigInt::from_i32(value.den);

    if numer.is_none() || denom.is_none() {
        Err(FFmpegError::InvalidData)
    } else {
        Ok(BigRational::new(numer.unwrap(), denom.unwrap()))
    }
}

pub fn reduce_bigrational_to_i32(value: &BigRational) -> BigRational {
    let mut result = value.clone();
    let ten = BigInt::from_usize(10).unwrap();

    while result.numer().to_i32().is_none() || result.denom().to_i32().is_none() {
        result = BigRational::from((result.numer() / &ten, result.denom() / &ten));
    }

    result
}

pub fn cstring_from_path<P: AsRef<Path>>(path: P) -> Result<CString, FFmpegError> {
    let path_str = path.as_ref().as_os_str().to_str();
    if path_str.is_some() {
        let path_cstr = CString::new(path_str.unwrap());
        if path_cstr.is_ok() {
            return Ok(path_cstr.unwrap());
        }
    }

    Err(FFmpegError::InvalidData)
}

pub fn init() {
    unsafe {
        ff::av_register_all();
    }
}

pub fn str_from_utf8_cstr_unchecked<'a>(ptr: *const c_char) -> &'a str {
    unsafe {
        from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
    }
}

#[derive(Debug)]
pub enum FFmpegError {
    // FFmpeg errors
    BitStreamFilterNotFound,
    BufferTooSmall,
    Bug,
    Bug2,
    DecoderNotFound,
    DemuxerNotFound,
    EncoderNotFound,
    EOF,
    Exit,
    Experimental,
    External,
    FilterNotFound,
    HttpBadRequest,
    HttpForbidden,
    HttpNotFound,
    HttpOther4xx,
    HttpServerError,
    HttpUnauthorized,
    InputChanged,
    InvalidData,
    MuxerNotFound,
    OptionNotFound,
    OutputChanged,
    PatchWelcome,
    ProtocolNotFound,
    StreamNotFound,

    // libc errors
    NoEntry, // AKA File not found

    Unknown(i32),
}

impl From<i32> for FFmpegError {
    fn from(err: i32) -> Self {
        match err {
            ff::AVERROR_BSF_NOT_FOUND => BitStreamFilterNotFound,
            ff::AVERROR_BUG => Bug,
            ff::AVERROR_BUG2 => Bug2,
            ff::AVERROR_BUFFER_TOO_SMALL => BufferTooSmall,
            ff::AVERROR_DECODER_NOT_FOUND => DecoderNotFound,
            ff::AVERROR_DEMUXER_NOT_FOUND => DemuxerNotFound,
            ff::AVERROR_ENCODER_NOT_FOUND => EncoderNotFound,
            ff::AVERROR_EOF => EOF,
            ff::AVERROR_EXIT => Exit,
            ff::AVERROR_EXPERIMENTAL => Experimental,
            ff::AVERROR_EXTERNAL => External,
            ff::AVERROR_FILTER_NOT_FOUND => FilterNotFound,
            ff::AVERROR_HTTP_BAD_REQUEST => HttpBadRequest,
            ff::AVERROR_HTTP_FORBIDDEN => HttpForbidden,
            ff::AVERROR_HTTP_NOT_FOUND => HttpNotFound,
            ff::AVERROR_HTTP_OTHER_4XX => HttpOther4xx,
            ff::AVERROR_HTTP_SERVER_ERROR => HttpServerError,
            ff::AVERROR_HTTP_UNAUTHORIZED => HttpUnauthorized,
            ff::AVERROR_INPUT_CHANGED => InputChanged,
            ff::AVERROR_INVALIDDATA => InvalidData,
            ff::AVERROR_MUXER_NOT_FOUND => MuxerNotFound,
            ff::AVERROR_OPTION_NOT_FOUND => OptionNotFound,
            ff::AVERROR_OUTPUT_CHANGED => OutputChanged,
            ff::AVERROR_PATCHWELCOME => PatchWelcome,
            ff::AVERROR_PROTOCOL_NOT_FOUND => ProtocolNotFound,
            ff::AVERROR_STREAM_NOT_FOUND => StreamNotFound,
            ff::AVERROR_UNKNOWN => Unknown(0),
            e if libc::ENOENT == ff::AVUNERROR(e) => NoEntry,
            _ => Unknown(err),
        }
    }
}