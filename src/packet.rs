use std::ops::{Deref, DerefMut};

use ::ff;

pub struct Packet(*mut ff::AVPacket);

impl Packet {
    pub fn alloc() -> Self {
        unsafe {
            match ff::av_packet_alloc() {
                packet if !packet.is_null() => Packet(packet),
                _ => panic!("out of memory"),
            }
        }
    }

    pub fn dts(&self) -> i64 {
        unsafe {
            (*self.0).dts
        }
    }

    pub fn duration(&self) -> i64 {
        unsafe {
            (*self.0).duration
        }
    }

    pub fn pos(&self) -> i64 {
        unsafe {
            (*self.0).pos
        }
    }

    pub fn pts(&self) -> i64 {
        unsafe {
            (*self.0).pts
        }
    }

    pub fn set_dts(&mut self, value: i64) {
        unsafe {
            (*self.0).dts = value;
        }
    }

    pub fn set_duration(&mut self, value: i64) {
        unsafe {
            (*self.0).duration = value;
        }
    }

    pub fn set_pos(&mut self, value: i64) {
        unsafe {
            (*self.0).pos = value;
        }
    }

    pub fn set_pts(&mut self, value: i64) {
        unsafe {
            (*self.0).pts = value;
        }
    }

    pub fn set_stream_index(&mut self, value: usize) {
        unsafe {
            (*self.0).stream_index = value as i32;
        }
    }

    pub fn stream_index(&self) -> usize {
        unsafe {
            (*self.0).stream_index as usize
        }
    }
}

impl Deref for Packet {
    type Target = ff::AVPacket;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0
        }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            ff::av_packet_unref(&mut *self.0);
            ff::av_packet_free(&mut self.0);
        }
    }
}