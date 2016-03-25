#![feature(unique)]
extern crate libc;
mod bindgen;
use bindgen::{SF_INFO, SNDFILE, sf_count_t, sf_open, sf_readf_float, sf_close, sf_error, sf_error_number, sf_strerror, OpenMode, SfError};

use std::ffi::{CString, NulError, CStr};
use libc::{c_int, c_float};
use std::ptr::Unique;

/// Information about a sound file.
///
/// Currently a raw copy of the `SF_INFO` struct - better API tbc.
/// Check out [the libsndfile docs](http://www.mega-nerd.com/libsndfile/api.html#open)
/// for more info on this struct.
#[derive(Debug, Clone, Copy)]
pub struct SndFileInfo {
    /// The number of frames in the file
    pub frames: sf_count_t,
    /// The file's sample rate
    pub samplerate: c_int,
    /// The number of channels in the file
    pub channels: c_int,
    /// The file's format, which is currently gibberish
    format: c_int,
    /// I have no idea what this is
    sections: c_int,
    /// Whether the file is seekable, but I don't know what's true and what's false
    seekable: c_int
}
impl From<SF_INFO> for SndFileInfo {
    fn from(info: SF_INFO) -> Self {
        SndFileInfo {
            frames: info.frames,
            samplerate: info.samplerate,
            channels: info.channels,
            format: info.format,
            sections: info.sections,
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
    /// writing is not supported yet.

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

    /// Reads sound data from a sound file into a buffer of `libc::c_float`s.
    ///
    /// Returns either the amount of frames actually read (which may be less than
    /// the `frames` requested if there are less frames available).
    /// A `c_float` is usually `f32` on most platforms.
    ///
    /// # Panics
    ///
    /// Panics if the amount of frames read exceeds `usize::MAX` or is less than 0.

    pub fn into_slice_float(&mut self, buf: &mut [c_float], frames: usize) -> Result<usize, SndError> {
        let units_reqd = self.info.channels as usize * frames;
        if buf.len() < units_reqd {
            return Err(SndError {
                err: SfError::SF_ERR_UNLISTED as c_int,
                expl: "Binding: Buffer provided not big enough".to_string()
            });
        }
        let mut written: sf_count_t = 0;
        let ptr = buf.as_mut_ptr();
        unsafe {
            written = sf_readf_float(*self.ptr, ptr, frames as i64);
        }
        assert!(written >= 0);
        assert!((written as usize) < ::std::usize::MAX);
        Ok(written as usize)
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

