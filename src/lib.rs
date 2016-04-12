#![feature(unique)]
extern crate libc;
mod bindgen;
use bindgen::{SF_INFO,
              SNDFILE,
              sf_count_t,
              sf_open,
              sf_readf_float,
              sf_close,
              sf_error,
              sf_error_number,
              sf_strerror,
              sf_seek,
              OpenMode,
              SfError};
use bindgen::Seek as SfSeek;
use std::ffi::{CString, NulError, CStr};
use libc::{c_int};
use std::io::Seek;
pub use std::io::SeekFrom;
use std::ptr::Unique;
use std::io::Result as IOResult;
use std::io::Error as IOError;
/// Information about a sound file.
///
/// Currently a raw copy of the `SF_INFO` struct - better API tbc.
/// Check out [the libsndfile docs](http://www.mega-nerd.com/libsndfile/api.html#open)
/// for more info on this struct.
#[derive(Debug, Clone, Copy)]
pub struct SndFileInfo {
    /// The number of frames in the file
    pub frames: u64,
    /// The file's sample rate
    pub samplerate: u32,
    /// The number of channels in the file
    pub channels: u16,
    /// Whether the file is seekable, but I don't know what's true and what's false
    seekable: c_int
}
impl From<SF_INFO> for SndFileInfo {
    fn from(info: SF_INFO) -> Self {
        assert!(info.frames as u64 <= ::std::u64::MAX && info.frames >= 0);
        assert!(info.samplerate as u32 <= ::std::u32::MAX && info.samplerate >= 0);
        assert!(info.channels as u16 <= ::std::u16::MAX && info.channels >= 0);
        SndFileInfo {
            frames: info.frames as u64,
            samplerate: info.samplerate as u32,
            channels: info.channels as u16,
            seekable: info.seekable
        }
    }
}
/// A `libsndfile` sound file object.
pub struct SndFile {
    ptr: Unique<SNDFILE>,
    pub info: SndFileInfo
}
impl Drop for SndFile {
    fn drop(&mut self) {
        unsafe { sf_close(*self.ptr) };
    }
}
impl SndFile {
    /// Opens a sound file for reading.
    ///
    /// Takes a `path` to the sound file to open. Please note that opening for
    /// writing is not supported yet, so this just opens for reading.

    pub fn open(path: &str) -> Result<Self, SndError> {
        let cstr = try!(CString::new(path));
        let mut sfi = SF_INFO {
            frames: 0,
            samplerate: 0,
            channels: 0,
            format: 0,
            sections: 0,
            seekable: 0
        };
        let sndfile_ptr: *mut SNDFILE;
        unsafe {
            sndfile_ptr = sf_open(cstr.as_ptr(), OpenMode::SFM_READ as c_int, &mut sfi);
        }
        if sndfile_ptr.is_null() {
            Err(Self::get_error(sndfile_ptr))
        }
        else {
            let unique: Unique<SNDFILE>;
            unsafe {
                unique = Unique::new(sndfile_ptr);
            }
            Ok(SndFile {
                ptr: unique,
                info: SndFileInfo::from(sfi)
            })
        }
    }
    fn get_error(sf: *mut SNDFILE) -> SndError {
        let strptr: &CStr;
        let mut errno: c_int;
        unsafe {
            errno = sf_error(sf);
            if errno >= SfError::SF_ERR_UNLISTED as c_int {
                strptr = CStr::from_ptr(sf_error_number(errno));
                errno = SfError::SF_ERR_UNLISTED as c_int;
            }
            else {
                strptr = CStr::from_ptr(sf_strerror(sf));
            }
        }
        let expl = strptr.to_string_lossy().into_owned();
        SndError {
            err: errno,
            expl: expl
        }
    }

    /// Reads frames of sound data from a sound file into a buffer of f32s.
    ///
    /// The buffer provided must be as long as (frames wanted * channels in file),
    /// as it reads **interleaved** sound data. This looks something like:
    ///
    /// ```text
    /// interleaved data:
    /// buffer  [0.0, 0.2, 0.3, 0.4, 0.5, 0.8]
    /// channel   1    2    1    2    1    2
    /// index     0    0    1    1    2    2
    ///
    /// hence, non-interleaved channel 1:
    /// [0.0, 0.3, 0.5]
    /// non-interleaved channel 2:
    /// [0.2, 0.4, 0.8]
    /// ```
    ///
    /// Returns the number of interleaved frames read into the buffer. *Note:*
    /// This number may be less than what you expected! This can be caused by:
    ///
    /// - You supplying a buffer that doesn't divide evenly (a buf of length 3 cannot take
    ///   2 frames of interleaved data, only 1, so buf[2] will not be written to)
    /// - You reaching the end of the file.
    /// - Solar flares/extraterrestrial activity.
    ///
    /// # Panics
    ///
    /// Panics if the amount of frames read exceeds `u64::MAX` or is less than 0.

    pub fn read_into_fslice(&mut self, buf: &mut [f32]) -> u64 {
        let written: sf_count_t;
        let ptr = buf.as_mut_ptr();
        unsafe {
            written = sf_readf_float(*self.ptr, ptr, buf.len() as i64 / self.info.channels as i64);
        }
        assert!(written >= 0, "rsndfile read_into_fslice: negative frames written");
        assert!((written as u64) < ::std::u64::MAX, "rsndfile read_into_fslice: written >= u64 MAX");
        written as u64
    }

}
impl Seek for SndFile {
    fn seek(&mut self, pos: SeekFrom) -> IOResult<u64> {
        let frames: sf_count_t;
        let whence: c_int;
        match pos {
            SeekFrom::Start(f) => {
                whence = SfSeek::SF_SEEK_SET as c_int;
                frames = f as i64;
            },
            SeekFrom::End(f) => {
                whence = SfSeek::SF_SEEK_END as c_int;
                frames = f;
            },
            SeekFrom::Current(f) => {
                whence = SfSeek::SF_SEEK_CUR as c_int;
                frames = f;
            }
        }
        let new_pos: sf_count_t;
        unsafe {
            new_pos = sf_seek(*self.ptr, frames, whence);
        }
        if new_pos < 0 {
            Err(IOError::new(::std::io::ErrorKind::Other, "libsndfile returned -1 while seeking"))
        }
        else {
            Ok(new_pos as u64)
        }
    }
}

/// An error type.
///
/// This one sucks. A better one is coming.
#[derive(Debug)]
pub struct SndError {
    err: c_int,
    pub expl: String
}
impl From<NulError> for SndError {
    fn from(_: NulError) -> SndError {
        SndError {
            err: SfError::SF_ERR_UNLISTED as c_int,
            expl: "Binding: Encountered a NUL byte in path argument".to_string()
        }
    }
}

