/* automatically generated by rust-bindgen */
#![allow(dead_code, non_camel_case_types)]
extern crate libc;
use libc::*;
pub const SF_FORMAT_ENDMASK: FileFormat = FileFormat::SF_ENDIAN_CPU;
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum FileFormat {
    SF_FORMAT_WAV = 65536,
    SF_FORMAT_AIFF = 131072,
    SF_FORMAT_AU = 196608,
    SF_FORMAT_RAW = 262144,
    SF_FORMAT_PAF = 327680,
    SF_FORMAT_SVX = 393216,
    SF_FORMAT_NIST = 458752,
    SF_FORMAT_VOC = 524288,
    SF_FORMAT_IRCAM = 655360,
    SF_FORMAT_W64 = 720896,
    SF_FORMAT_MAT4 = 786432,
    SF_FORMAT_MAT5 = 851968,
    SF_FORMAT_PVF = 917504,
    SF_FORMAT_XI = 983040,
    SF_FORMAT_HTK = 1048576,
    SF_FORMAT_SDS = 1114112,
    SF_FORMAT_AVR = 1179648,
    SF_FORMAT_WAVEX = 1245184,
    SF_FORMAT_SD2 = 1441792,
    SF_FORMAT_FLAC = 1507328,
    SF_FORMAT_CAF = 1572864,
    SF_FORMAT_WVE = 1638400,
    SF_FORMAT_OGG = 2097152,
    SF_FORMAT_MPC2K = 2162688,
    SF_FORMAT_RF64 = 2228224,
    SF_FORMAT_PCM_S8 = 1,
    SF_FORMAT_PCM_16 = 2,
    SF_FORMAT_PCM_24 = 3,
    SF_FORMAT_PCM_32 = 4,
    SF_FORMAT_PCM_U8 = 5,
    SF_FORMAT_FLOAT = 6,
    SF_FORMAT_DOUBLE = 7,
    SF_FORMAT_ULAW = 16,
    SF_FORMAT_ALAW = 17,
    SF_FORMAT_IMA_ADPCM = 18,
    SF_FORMAT_MS_ADPCM = 19,
    SF_FORMAT_GSM610 = 32,
    SF_FORMAT_VOX_ADPCM = 33,
    SF_FORMAT_G721_32 = 48,
    SF_FORMAT_G723_24 = 49,
    SF_FORMAT_G723_40 = 50,
    SF_FORMAT_DWVW_12 = 64,
    SF_FORMAT_DWVW_16 = 65,
    SF_FORMAT_DWVW_24 = 66,
    SF_FORMAT_DWVW_N = 67,
    SF_FORMAT_DPCM_8 = 80,
    SF_FORMAT_DPCM_16 = 81,
    SF_FORMAT_VORBIS = 96,
    SF_FORMAT_ALAC_16 = 112,
    SF_FORMAT_ALAC_20 = 113,
    SF_FORMAT_ALAC_24 = 114,
    SF_FORMAT_ALAC_32 = 115,
    SF_ENDIAN_FILE = 0,
    SF_ENDIAN_LITTLE = 268435456,
    SF_ENDIAN_BIG = 536870912,
    SF_ENDIAN_CPU = 805306368,
    SF_FORMAT_SUBMASK = 65535,
    SF_FORMAT_TYPEMASK = 268369920,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed2 {
    SFC_GET_LIB_VERSION = 4096,
    SFC_GET_LOG_INFO = 4097,
    SFC_GET_CURRENT_SF_INFO = 4098,
    SFC_GET_NORM_DOUBLE = 4112,
    SFC_GET_NORM_FLOAT = 4113,
    SFC_SET_NORM_DOUBLE = 4114,
    SFC_SET_NORM_FLOAT = 4115,
    SFC_SET_SCALE_FLOAT_INT_READ = 4116,
    SFC_SET_SCALE_INT_FLOAT_WRITE = 4117,
    SFC_GET_SIMPLE_FORMAT_COUNT = 4128,
    SFC_GET_SIMPLE_FORMAT = 4129,
    SFC_GET_FORMAT_INFO = 4136,
    SFC_GET_FORMAT_MAJOR_COUNT = 4144,
    SFC_GET_FORMAT_MAJOR = 4145,
    SFC_GET_FORMAT_SUBTYPE_COUNT = 4146,
    SFC_GET_FORMAT_SUBTYPE = 4147,
    SFC_CALC_SIGNAL_MAX = 4160,
    SFC_CALC_NORM_SIGNAL_MAX = 4161,
    SFC_CALC_MAX_ALL_CHANNELS = 4162,
    SFC_CALC_NORM_MAX_ALL_CHANNELS = 4163,
    SFC_GET_SIGNAL_MAX = 4164,
    SFC_GET_MAX_ALL_CHANNELS = 4165,
    SFC_SET_ADD_PEAK_CHUNK = 4176,
    SFC_SET_ADD_HEADER_PAD_CHUNK = 4177,
    SFC_UPDATE_HEADER_NOW = 4192,
    SFC_SET_UPDATE_HEADER_AUTO = 4193,
    SFC_FILE_TRUNCATE = 4224,
    SFC_SET_RAW_START_OFFSET = 4240,
    SFC_SET_DITHER_ON_WRITE = 4256,
    SFC_SET_DITHER_ON_READ = 4257,
    SFC_GET_DITHER_INFO_COUNT = 4258,
    SFC_GET_DITHER_INFO = 4259,
    SFC_GET_EMBED_FILE_INFO = 4272,
    SFC_SET_CLIPPING = 4288,
    SFC_GET_CLIPPING = 4289,
    SFC_GET_INSTRUMENT = 4304,
    SFC_SET_INSTRUMENT = 4305,
    SFC_GET_LOOP_INFO = 4320,
    SFC_GET_BROADCAST_INFO = 4336,
    SFC_SET_BROADCAST_INFO = 4337,
    SFC_GET_CHANNEL_MAP_INFO = 4352,
    SFC_SET_CHANNEL_MAP_INFO = 4353,
    SFC_RAW_DATA_NEEDS_ENDSWAP = 4368,
    SFC_WAVEX_SET_AMBISONIC = 4608,
    SFC_WAVEX_GET_AMBISONIC = 4609,
    SFC_RF64_AUTO_DOWNGRADE = 4624,
    SFC_SET_VBR_ENCODING_QUALITY = 4864,
    SFC_SET_COMPRESSION_LEVEL = 4865,
    SFC_SET_CART_INFO = 5120,
    SFC_GET_CART_INFO = 5121,
    SFC_TEST_IEEE_FLOAT_REPLACE = 24577,
    SFC_SET_ADD_DITHER_ON_WRITE = 4208,
    SFC_SET_ADD_DITHER_ON_READ = 4209,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed3 {
    SF_STR_TITLE = 1,
    SF_STR_COPYRIGHT = 2,
    SF_STR_SOFTWARE = 3,
    SF_STR_ARTIST = 4,
    SF_STR_COMMENT = 5,
    SF_STR_DATE = 6,
    SF_STR_ALBUM = 7,
    SF_STR_LICENSE = 8,
    SF_STR_TRACKNUMBER = 9,
    SF_STR_GENRE = 16,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub enum OpenMode {
    SF_FALSE = 0,
    SF_TRUE = 1,
    SFM_READ = 16,
    SFM_WRITE = 32,
    SFM_RDWR = 48,
    SF_AMBISONIC_NONE = 64,
    SF_AMBISONIC_B_FORMAT = 65,
}
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum SfError {
    SF_ERR_NO_ERROR = 0,
    SF_ERR_UNRECOGNISED_FORMAT = 1,
    SF_ERR_SYSTEM = 2,
    SF_ERR_MALFORMED_FILE = 3,
    SF_ERR_UNSUPPORTED_ENCODING = 4,
    SF_ERR_UNLISTED = 5
}
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed6 {
    SF_CHANNEL_MAP_INVALID = 0,
    SF_CHANNEL_MAP_MONO = 1,
    SF_CHANNEL_MAP_LEFT = 2,
    SF_CHANNEL_MAP_RIGHT = 3,
    SF_CHANNEL_MAP_CENTER = 4,
    SF_CHANNEL_MAP_FRONT_LEFT = 5,
    SF_CHANNEL_MAP_FRONT_RIGHT = 6,
    SF_CHANNEL_MAP_FRONT_CENTER = 7,
    SF_CHANNEL_MAP_REAR_CENTER = 8,
    SF_CHANNEL_MAP_REAR_LEFT = 9,
    SF_CHANNEL_MAP_REAR_RIGHT = 10,
    SF_CHANNEL_MAP_LFE = 11,
    SF_CHANNEL_MAP_FRONT_LEFT_OF_CENTER = 12,
    SF_CHANNEL_MAP_FRONT_RIGHT_OF_CENTER = 13,
    SF_CHANNEL_MAP_SIDE_LEFT = 14,
    SF_CHANNEL_MAP_SIDE_RIGHT = 15,
    SF_CHANNEL_MAP_TOP_CENTER = 16,
    SF_CHANNEL_MAP_TOP_FRONT_LEFT = 17,
    SF_CHANNEL_MAP_TOP_FRONT_RIGHT = 18,
    SF_CHANNEL_MAP_TOP_FRONT_CENTER = 19,
    SF_CHANNEL_MAP_TOP_REAR_LEFT = 20,
    SF_CHANNEL_MAP_TOP_REAR_RIGHT = 21,
    SF_CHANNEL_MAP_TOP_REAR_CENTER = 22,
    SF_CHANNEL_MAP_AMBISONIC_B_W = 23,
    SF_CHANNEL_MAP_AMBISONIC_B_X = 24,
    SF_CHANNEL_MAP_AMBISONIC_B_Y = 25,
    SF_CHANNEL_MAP_AMBISONIC_B_Z = 26,
    SF_CHANNEL_MAP_MAX = 27,
}

pub enum Struct_SNDFILE_tag { }
pub type SNDFILE = Struct_SNDFILE_tag;
pub type sf_count_t = int64_t;
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_SF_INFO {
    pub frames: sf_count_t,
    pub samplerate: c_int,
    pub channels: c_int,
    pub format: c_int,
    pub sections: c_int,
    pub seekable: c_int,
}
impl ::std::clone::Clone for Struct_SF_INFO {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_SF_INFO {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_INFO = Struct_SF_INFO;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub format: c_int,
    pub name: *const c_char,
    pub extension: *const c_char,
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_FORMAT_INFO = Struct_Unnamed7;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Enum_Unnamed8 {
    SFD_DEFAULT_LEVEL = 0,
    SFD_CUSTOM_LEVEL = 1073741824,
    SFD_NO_DITHER = 500,
    SFD_WHITE = 501,
    SFD_TRIANGULAR_PDF = 502,
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub _type: c_int,
    pub level: c_double,
    pub name: *const c_char,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_DITHER_INFO = Struct_Unnamed9;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed10 {
    pub offset: sf_count_t,
    pub length: sf_count_t,
}
impl ::std::clone::Clone for Struct_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_EMBED_FILE_INFO = Struct_Unnamed10;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum FileFormat1 {
    SF_LOOP_NONE = 800,
    SF_LOOP_FORWARD = 801,
    SF_LOOP_BACKWARD = 802,
    SF_LOOP_ALTERNATING = 803,
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub gain: c_int,
    pub basenote: c_char,
    pub detune: c_char,
    pub velocity_lo: c_char,
    pub velocity_hi: c_char,
    pub key_lo: c_char,
    pub key_hi: c_char,
    pub loop_count: c_int,
    pub loops: [Struct_Unnamed13; 16usize],
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed13 {
    pub mode: c_int,
    pub start: uint32_t,
    pub end: uint32_t,
    pub count: uint32_t,
}
impl ::std::clone::Clone for Struct_Unnamed13 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed13 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_INSTRUMENT = Struct_Unnamed12;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub time_sig_num: c_short,
    pub time_sig_den: c_short,
    pub loop_mode: c_int,
    pub num_beats: c_int,
    pub bpm: c_float,
    pub root_key: c_int,
    pub future: [c_int; 6usize],
}
impl ::std::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_LOOP_INFO = Struct_Unnamed14;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed15 {
    pub description: [c_char; 256usize],
    pub originator: [c_char; 32usize],
    pub originator_reference: [c_char; 32usize],
    pub origination_date: [c_char; 10usize],
    pub origination_time: [c_char; 8usize],
    pub time_reference_low: uint32_t,
    pub time_reference_high: uint32_t,
    pub version: c_short,
    pub umid: [c_char; 64usize],
    pub reserved: [c_char; 190usize],
    pub coding_history_size: uint32_t,
    pub coding_history: [c_char; 256usize],
}
impl ::std::clone::Clone for Struct_Unnamed15 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed15 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_BROADCAST_INFO = Struct_Unnamed15;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_SF_CART_TIMER {
    pub usage: [c_char; 4usize],
    pub value: int32_t,
}
impl ::std::clone::Clone for Struct_SF_CART_TIMER {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_SF_CART_TIMER {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_CART_TIMER = Struct_SF_CART_TIMER;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed16 {
    pub version: [c_char; 4usize],
    pub title: [c_char; 64usize],
    pub artist: [c_char; 64usize],
    pub cut_id: [c_char; 64usize],
    pub client_id: [c_char; 64usize],
    pub category: [c_char; 64usize],
    pub classification: [c_char; 64usize],
    pub out_cue: [c_char; 64usize],
    pub start_date: [c_char; 10usize],
    pub start_time: [c_char; 8usize],
    pub end_date: [c_char; 10usize],
    pub end_time: [c_char; 8usize],
    pub producer_app_id: [c_char; 64usize],
    pub producer_app_version: [c_char; 64usize],
    pub user_def: [c_char; 64usize],
    pub level_reference: int32_t,
    pub post_timers: [SF_CART_TIMER; 8usize],
    pub reserved: [c_char; 276usize],
    pub url: [c_char; 1024usize],
    pub tag_text_size: uint32_t,
    pub tag_text: [c_char; 256usize],
}
impl ::std::clone::Clone for Struct_Unnamed16 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed16 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_CART_INFO = Struct_Unnamed16;
pub type sf_vio_get_filelen =
    ::std::option::Option<unsafe extern "C" fn(user_data:
                                                   *mut c_void)
                              -> sf_count_t>;
pub type sf_vio_seek =
    ::std::option::Option<unsafe extern "C" fn(offset: sf_count_t,
                                               whence: c_int,
                                               user_data:
                                                   *mut c_void)
                              -> sf_count_t>;
pub type sf_vio_read =
    ::std::option::Option<unsafe extern "C" fn(ptr:
                                                   *mut c_void,
                                               count: sf_count_t,
                                               user_data:
                                                   *mut c_void)
                              -> sf_count_t>;
pub type sf_vio_write =
    ::std::option::Option<unsafe extern "C" fn(ptr:
                                                   *const c_void,
                                               count: sf_count_t,
                                               user_data:
                                                   *mut c_void)
                              -> sf_count_t>;
pub type sf_vio_tell =
    ::std::option::Option<unsafe extern "C" fn(user_data:
                                                   *mut c_void)
                              -> sf_count_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_SF_VIRTUAL_IO {
    pub get_filelen: sf_vio_get_filelen,
    pub seek: sf_vio_seek,
    pub read: sf_vio_read,
    pub write: sf_vio_write,
    pub tell: sf_vio_tell,
}
impl ::std::clone::Clone for Struct_SF_VIRTUAL_IO {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_SF_VIRTUAL_IO {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_VIRTUAL_IO = Struct_SF_VIRTUAL_IO;
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Seek { SF_SEEK_SET = 0, SF_SEEK_CUR = 1, SF_SEEK_END = 2, }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_SF_CHUNK_INFO {
    pub id: [c_char; 64usize],
    pub id_size: c_uint,
    pub datalen: c_uint,
    pub data: *mut c_void,
}
impl ::std::clone::Clone for Struct_SF_CHUNK_INFO {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_SF_CHUNK_INFO {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type SF_CHUNK_INFO = Struct_SF_CHUNK_INFO;
pub enum Struct_SF_CHUNK_ITERATOR { }
pub type SF_CHUNK_ITERATOR = Struct_SF_CHUNK_ITERATOR;
#[link(name = "sndfile")]
extern "C" {
    pub fn sf_open(path: *const c_char,
                   mode: c_int, sfinfo: *mut SF_INFO)
     -> *mut SNDFILE;
    pub fn sf_open_fd(fd: c_int, mode: c_int,
                      sfinfo: *mut SF_INFO, close_desc: c_int)
     -> *mut SNDFILE;
    pub fn sf_open_virtual(sfvirtual: *mut SF_VIRTUAL_IO,
                           mode: c_int, sfinfo: *mut SF_INFO,
                           user_data: *mut c_void)
     -> *mut SNDFILE;
    pub fn sf_error(sndfile: *mut SNDFILE) -> c_int;
    pub fn sf_strerror(sndfile: *mut SNDFILE)
     -> *const c_char;
    pub fn sf_error_number(errnum: c_int)
     -> *const c_char;
    pub fn sf_perror(sndfile: *mut SNDFILE) -> c_int;
    pub fn sf_error_str(sndfile: *mut SNDFILE,
                        str: *mut c_char, len: size_t)
     -> c_int;
    pub fn sf_command(sndfile: *mut SNDFILE, command: c_int,
                      data: *mut c_void,
                      datasize: c_int)
     -> c_int;
    pub fn sf_format_check(info: *const SF_INFO) -> c_int;
    pub fn sf_seek(sndfile: *mut SNDFILE, frames: sf_count_t,
                   whence: c_int) -> sf_count_t;
    pub fn sf_set_string(sndfile: *mut SNDFILE,
                         str_type: c_int,
                         str: *const c_char)
     -> c_int;
    pub fn sf_get_string(sndfile: *mut SNDFILE,
                         str_type: c_int)
     -> *const c_char;
    pub fn sf_version_string() -> *const c_char;
    pub fn sf_current_byterate(sndfile: *mut SNDFILE)
     -> c_int;
    pub fn sf_read_raw(sndfile: *mut SNDFILE,
                       ptr: *mut c_void, bytes: sf_count_t)
     -> sf_count_t;
    pub fn sf_write_raw(sndfile: *mut SNDFILE,
                        ptr: *const c_void, bytes: sf_count_t)
     -> sf_count_t;
    pub fn sf_readf_short(sndfile: *mut SNDFILE,
                          ptr: *mut c_short,
                          frames: sf_count_t) -> sf_count_t;
    pub fn sf_writef_short(sndfile: *mut SNDFILE,
                           ptr: *const c_short,
                           frames: sf_count_t) -> sf_count_t;
    pub fn sf_readf_int(sndfile: *mut SNDFILE,
                        ptr: *mut c_int, frames: sf_count_t)
     -> sf_count_t;
    pub fn sf_writef_int(sndfile: *mut SNDFILE,
                         ptr: *const c_int,
                         frames: sf_count_t) -> sf_count_t;
    pub fn sf_readf_float(sndfile: *mut SNDFILE,
                          ptr: *mut c_float,
                          frames: sf_count_t) -> sf_count_t;
    pub fn sf_writef_float(sndfile: *mut SNDFILE,
                           ptr: *const c_float,
                           frames: sf_count_t) -> sf_count_t;
    pub fn sf_readf_double(sndfile: *mut SNDFILE,
                           ptr: *mut c_double,
                           frames: sf_count_t) -> sf_count_t;
    pub fn sf_writef_double(sndfile: *mut SNDFILE,
                            ptr: *const c_double,
                            frames: sf_count_t) -> sf_count_t;
    pub fn sf_read_short(sndfile: *mut SNDFILE,
                         ptr: *mut c_short, items: sf_count_t)
     -> sf_count_t;
    pub fn sf_write_short(sndfile: *mut SNDFILE,
                          ptr: *const c_short,
                          items: sf_count_t) -> sf_count_t;
    pub fn sf_read_int(sndfile: *mut SNDFILE, ptr: *mut c_int,
                       items: sf_count_t) -> sf_count_t;
    pub fn sf_write_int(sndfile: *mut SNDFILE,
                        ptr: *const c_int, items: sf_count_t)
     -> sf_count_t;
    pub fn sf_read_float(sndfile: *mut SNDFILE,
                         ptr: *mut c_float, items: sf_count_t)
     -> sf_count_t;
    pub fn sf_write_float(sndfile: *mut SNDFILE,
                          ptr: *const c_float,
                          items: sf_count_t) -> sf_count_t;
    pub fn sf_read_double(sndfile: *mut SNDFILE,
                          ptr: *mut c_double,
                          items: sf_count_t) -> sf_count_t;
    pub fn sf_write_double(sndfile: *mut SNDFILE,
                           ptr: *const c_double,
                           items: sf_count_t) -> sf_count_t;
    pub fn sf_close(sndfile: *mut SNDFILE) -> c_int;
    pub fn sf_write_sync(sndfile: *mut SNDFILE);
    pub fn sf_set_chunk(sndfile: *mut SNDFILE,
                        chunk_info: *const SF_CHUNK_INFO)
     -> c_int;
    pub fn sf_get_chunk_iterator(sndfile: *mut SNDFILE,
                                 chunk_info: *const SF_CHUNK_INFO)
     -> *mut SF_CHUNK_ITERATOR;
    pub fn sf_next_chunk_iterator(iterator: *mut SF_CHUNK_ITERATOR)
     -> *mut SF_CHUNK_ITERATOR;
    pub fn sf_get_chunk_size(it: *const SF_CHUNK_ITERATOR,
                             chunk_info: *mut SF_CHUNK_INFO)
     -> c_int;
    pub fn sf_get_chunk_data(it: *const SF_CHUNK_ITERATOR,
                             chunk_info: *mut SF_CHUNK_INFO)
     -> c_int;
}
