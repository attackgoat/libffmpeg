use std::ffi::CString;
use std::ops::Deref;

use ::{ff, str_from_utf8_cstr_unchecked};

pub struct Codec(*const ff::AVCodec);

impl Codec {
    pub fn find_encoder(codec_id: ff::AVCodecID) -> Option<Self> {
        unsafe {
            match ff::avcodec_find_encoder(codec_id) {
                codec if !codec.is_null() => Some(Codec(codec)),
                _ => None,
            }
        }
    }

    pub fn find_encoder_by_name(name: &str) -> Option<Self> {
        let name = CString::new(name).unwrap();
        unsafe {
            match ff::avcodec_find_encoder_by_name(name.as_ptr()) {
                codec if !codec.is_null() => Some(Codec(codec)),
                _ => None,
            }
        }
    }

    pub fn channel_layouts(&self) -> ChannelLayoutIter {
        unsafe {
            ChannelLayoutIter((*self.0).channel_layouts)
        }
    }

    pub fn name(&self) -> &str {
		unsafe {
			str_from_utf8_cstr_unchecked((*self.0).name)
		}
    }

    pub fn sample_formats(&self) -> SampleFormatIter {
        unsafe {
            SampleFormatIter((*self.0).sample_fmts)
        }
    }

    pub fn supported_sample_rates(&self) -> SupportedSampleRateIter {
        unsafe {
            SupportedSampleRateIter((*self.0).supported_samplerates)
        }
    }
}

impl Deref for Codec {
    type Target = *const ff::AVCodec;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<*const ff::AVCodec> for Codec {
    fn from(ptr: *const ff::AVCodec) -> Self {
        if ptr.is_null() {
            panic!("null AVCodec");
        }

        Codec(ptr)
    }
}

pub struct ChannelLayoutIter(*const u64);

impl Iterator for ChannelLayoutIter {
    type Item = u64;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.0.is_null() {
            return None;
        }

        unsafe {
            match *self.0 {
                0 => None,
                channel_layout => {
                    self.0 = self.0.offset(1);
                    Some(channel_layout)
                }
            }
        }
    }
}

pub struct SampleFormatIter(*const ff::AVSampleFormat);

impl Iterator for SampleFormatIter {
    type Item = ff::AVSampleFormat;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.0.is_null() {
            return None;
        }

        unsafe {
            match *self.0 {
                ff::AVSampleFormat::AV_SAMPLE_FMT_NONE => None,
                sample_format => {
                    self.0 = self.0.offset(1);
                    Some(sample_format)
                }
            }
        }
    }
}

pub struct SupportedSampleRateIter(*const i32);

impl Iterator for SupportedSampleRateIter {
    type Item = i32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.0.is_null() {
            return None;
        }
        
        unsafe {
            match *self.0 {
                0 => None,
                sample_rate => {
                    self.0 = self.0.offset(1);
                    Some(sample_rate)
                }
            }
        }
    }
}


/*fn test_codecs() -> Result<(), LibError> {
    test_codec(&Codec::find_encoder_by_name("libfdk_aac").unwrap());
    test_codec(&Codec::find_encoder(ff::AV_CODEC_ID_AAC).unwrap());
    Ok(())
}

fn test_codec(codec: &Codec) {
    println!("Codec: {}", codec.name());
    for channel_layout in codec.channel_layouts() {
        println!("Channel Layout: {}", channel_layout);
    }
    for sample_format in codec.sample_formats() {
        println!("Sample Format: {:?}", sample_format);
    }
    for supported_sample_rate in codec.supported_sample_rates() {
        println!("Supported Sample Rate: {}", supported_sample_rate);
    }
}*/