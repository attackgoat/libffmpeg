use std::path::Path;
use std::ptr::{null, null_mut};

use ::{Codec, cstring_from_path, ff, FFmpegError, InputFormat, OutputFormat, Packet, Stream};

pub struct FormatContext(*mut ff::AVFormatContext);

impl FormatContext {
    /// Opens a file for reading - unsafe because you must call close_input() manually
    pub unsafe fn open_read<P: AsRef<Path>>(filename: P) -> Result<Self, FFmpegError> {
        Self::open_read_as(filename, &mut InputFormat::from(null_mut()))
    }

    /// Opens a file for reading - unsafe because you must call close_input() manually
    pub unsafe fn open_read_as<P: AsRef<Path>>(filename: P, format: &mut ff::AVInputFormat) -> Result<Self, FFmpegError> {
        let mut format_context = null_mut();

        // Open the file (allocates format context for us) - no need to call close_input
        let filename = cstring_from_path(filename)?;
        match ff::avformat_open_input(&mut format_context, filename.as_ptr(), &mut *format, null_mut()) {
            0 => (),
            e => return Err(FFmpegError::from(e)),
        };

        #[cfg(debug_assertions)] {
            println!("Opening {:?} using input format {}", filename, InputFormat::from((*format_context).iformat).name());
        }

        // Make sure there is a header or stream info we can read
        match ff::avformat_find_stream_info(format_context, null_mut()) {
            e if 0 > e || format_context.is_null() => Err(FFmpegError::from(e)),
            _ => Ok(FormatContext(format_context)),
        }
    }

    /// Opens a file for writing - unsafe because you must call close_output() manually
    pub unsafe fn open_write<P: AsRef<Path>>(filename: P) -> Result<Self, FFmpegError> {
        Self::open_write_as(filename, &mut OutputFormat::from(null_mut()))
    }

    /// Opens a file for writing - unsafe because you must call close_output() manually
    pub unsafe fn open_write_as<P: AsRef<Path>>(filename: P, format: &mut ff::AVOutputFormat) -> Result<Self, FFmpegError> {
        let mut format_context = null_mut();
        let filename = cstring_from_path(filename)?;

        // Allocate the output context
        match ff::avformat_alloc_output_context2(&mut format_context, &mut *format, null(), filename.as_ptr()) {
            e if 0 > e || format_context.is_null() => return Err(FFmpegError::from(e)),
            _ => (),
        };

        #[cfg(debug_assertions)] {
            println!("Opening {:?} using output format {}", filename, OutputFormat::from((*format_context).oformat).name());
        }

        // Open the file - you must call close_output manually
        match ff::avio_open(&mut (*format_context).pb, filename.as_ptr(), ff::AVIO_FLAG_WRITE) {
            e if 0 > e => Err(FFmpegError::from(e)),
            _ => Ok(FormatContext(format_context)),
        }
    }

    /// Closes a file that was opened for reading - unsafe because you must be in read mode
    pub unsafe fn close_input(&mut self) {
        ff::avformat_close_input(&mut self.0);
    }

    /// Closes a file that was opened for writing - unsafe because you must be in write mode
    pub unsafe fn close_output(&mut self) -> Result<(), FFmpegError> {
        match ff::avio_close((*self.0).pb) {
            0 => Ok(()),
            e => Err(FFmpegError::from(e)),
        }
    }

    pub fn find_best_stream(&self, ty: ff::AVMediaType) -> Result<Option<(Codec, usize)>, FFmpegError> {
        let mut ptr = null_mut();
        unsafe {
            match ff::av_find_best_stream(self.0, ty, -1, -1, &mut ptr, 0) {
                index if -1 < index && !ptr.is_null() => Ok(Some((Codec::from(ptr as *const _), index as usize))),
                ff::AVERROR_STREAM_NOT_FOUND => Ok(None),
                e => Err(FFmpegError::from(e)),
            }
        }
    }

    pub fn flags(&self) -> i32 {
        unsafe {
            (*self.0).flags
        }
    }

    /// Adds a new stream to the context - unsafe because returned stream
    /// is only valid until close_output() and you must be in write mode
    pub unsafe fn new_stream(&mut self) -> Result<Stream, FFmpegError> {
        match ff::avformat_new_stream(self.0, null()) {
            stream if !stream.is_null() => Ok(Stream::from(stream)),
            _ => Err(FFmpegError::InvalidData),
        }
    }

    /// Adds a new stream to the context - unsafe because returned stream
    /// is only valid until close_output() and you must be in write mode
    /*pub unsafe fn new_stream_as(&mut self, codec: &Codec) -> Result<Stream, FFmpegError> {
        match ff::avformat_new_stream(self.0, **codec) {
            stream if !stream.is_null() => Ok(Stream::from(stream)),
            _ => Err(FFmpegError::InvalidData),
        }
    }*/

    /// Reads one frame - unsafe because returned packet is only
    /// valid until the next read_frame() or until close_input()
    pub unsafe fn read_frame(&mut self) -> Result<Option<Packet>, FFmpegError> {
        let mut packet = Packet::alloc();
        match ff::av_read_frame(self.0, &mut *packet) {
            ff::AVERROR_EOF => Ok(None),
            e if 0 > e => Err(FFmpegError::from(e)),
            _ => Ok(Some(packet)),
        }
    }

    /// Seeks to a timestamp - unsafe because you must be in read mode
    pub unsafe fn seek_byte(&mut self, stream_index: usize, byte: i64) -> Result<(), FFmpegError> {
        match ff::av_seek_frame(self.0, stream_index as i32, byte, ff::AVSEEK_FLAG_BYTE) {
            e if 0 > e => Err(FFmpegError::from(e)),
            _ => Ok(()),
        }
    }

    /// Seeks to a timestamp - unsafe because you must be in read mode
    pub unsafe fn seek_frame(&mut self, stream_index: usize, ts: i64) -> Result<(), FFmpegError> {
        match ff::av_seek_frame(self.0, stream_index as i32, ts, ff::AVSEEK_FLAG_FRAME | ff::AVSEEK_FLAG_BACKWARD) {
            e if 0 > e => Err(FFmpegError::from(e)),
            _ => Ok(()),
        }
    }

    /// Returns the given stream - unsafe because returned stream
    /// is only valid until close_input() or close_output()
    pub unsafe fn stream(&self, index: usize) -> Result<Option<Stream>, FFmpegError> {
        let ref format_context = *self.0;
        assert!(format_context.nb_streams as usize > index);
        Ok(Some(Stream::from(*format_context.streams.offset(index as isize))))
    }

    /// Writes file header - unsafe because you must be in write mode
    pub unsafe fn write_header(&mut self) -> Result<(), FFmpegError> {
        match ff::avformat_write_header(self.0, null_mut()) {
            e if 0 > e => Err(FFmpegError::from(e)),
            _ => Ok(())
        }
    }

    /// Writes file data - unsafe because you must be in write mode
    pub unsafe fn write_interleaved(&mut self, packet: &mut Packet) -> Result<(), FFmpegError> {
        match ff::av_interleaved_write_frame(self.0, &mut **packet) {
            e if 0 > e => Err(FFmpegError::from(e)),
            _ => Ok(()),
        }
    }

    /// Writes the file trailer - unsafe because you must be in write mode
    pub unsafe fn write_trailer(&mut self) -> Result<(), FFmpegError> {
        match ff::av_write_trailer(self.0) {
            e if 0 > e => Err(FFmpegError::from(e)),
            _ => Ok(()),
        }
    }
}

impl Drop for FormatContext {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ff::avformat_free_context(self.0);
        }
    }
}