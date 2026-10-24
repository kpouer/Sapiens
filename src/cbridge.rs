#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::shared::Rect;
use std::ffi::CString;
pub(crate) type __int64_t = ::std::os::raw::c_longlong;

pub(crate) type OSErr = i16;
pub(crate) type MenuHandle = *mut ::std::os::raw::c_void;
pub(crate) const everyEvent: u32 = 65535;
pub(crate) const helvetica: u32 = 2;
pub(crate) const itemDisable: u32 = 128;
pub(crate) const optionKey: u32 = 2048;
pub(crate) const statText: u32 = 8;
pub(crate) const radCtrl: u32 = 20;
pub(crate) const userItem: u32 = 32;

pub(crate) type TPPrPort = *mut ::std::os::raw::c_void;
pub(crate) const kOnSystemDisk: u32 = 32768;
pub(crate) const kPreferencesFolderType: u32 = 1886545254;
pub(crate) const kCreateFolder: u32 = 1;
pub(crate) type QElemPtr = *mut ::std::os::raw::c_void;
pub(crate) type SndChannelPtr = *mut ::std::os::raw::c_void;
pub(crate) const teJustLeft: u32 = 0;
pub(crate) const geneva: u32 = 0;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TEText {
    pub(crate) txFont: i16,
    pub(crate) txFace: i16,
    pub(crate) txMode: i16,
    pub(crate) txSize: i16,
    pub(crate) lineHeight: i16,
    pub(crate) fontAscent: i16,
    pub(crate) fontDescent: i16,
    pub(crate) nLines: i64,
}

pub(crate) type WindowPtr = *mut ::std::os::raw::c_void;

pub(crate) type __darwin_off_t = __int64_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TPrint {
    pub(crate) prInfo: TPrint__bindgen_ty_1,
    pub(crate) prJob: TPrint__bindgen_ty_2,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TPrint__bindgen_ty_1 {
    pub(crate) rPage: Rect,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TPrint__bindgen_ty_2 {
    pub(crate) bJDocLoop: i16,
}

pub(crate) type FILE = __sFILE;
pub(crate) const charCodeMask: u32 = 255;
pub(crate) const courier: u32 = 1;
pub(crate) const noErr: u32 = 0;
pub(crate) const cmdKey: u32 = 256;
pub(crate) const ctrlItem: u32 = 16;
pub(crate) const activateEvt: u32 = 8;
pub(crate) const app4Evt: u32 = 55;
pub(crate) const bold: u32 = 1;
pub(crate) const editText: u32 = 4;

pub(crate) const watchCursor: u32 = 0;
pub(crate) const keyDown: u32 = 3;
pub(crate) const autoKey: u32 = 5;
pub(crate) const keyUp: u32 = 4;
pub(crate) const blackColor: u32 = 33;
pub(crate) const whiteColor: u32 = 30;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct __sbuf {
    pub(crate) _base: *mut u8,
    pub(crate) _size: i32,
}
impl Default for __sbuf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub(crate) const patOr: u32 = 1;
pub(crate) const srcCopy: u32 = 0;
pub(crate) type THPrint = *mut *mut TPrint;

pub(crate) const DRVR: u32 = 1146246738;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct DateTimeRec {
    pub(crate) year: i16,
    pub(crate) month: i16,
    pub(crate) day: i16,
    pub(crate) hour: i16,
    pub(crate) minute: i16,
    pub(crate) second: i16,
    pub(crate) dayOfWeek: i16,
}
pub(crate) type fpos_t = __darwin_off_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct __sFILE {
    pub(crate) _p: *mut u8,
    pub(crate) _r: i32,
    pub(crate) _w: i32,
    pub(crate) _flags: i16,
    pub(crate) _file: i16,
    pub(crate) _bf: __sbuf,
    pub(crate) _lbfsize: i32,
    pub(crate) _cookie: *mut ::std::os::raw::c_void,
    pub(crate) _close:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> i32>,
    pub(crate) _read: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *mut i8, __n: i32) -> i32,
    >,
    pub(crate) _seek: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: fpos_t, arg3: i32) -> fpos_t,
    >,
    pub(crate) _write: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *const i8, __n: i32) -> i32,
    >,
}
impl Default for __sFILE {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct CInfoPBRec {
    pub(crate) hFileInfo: CInfoPBRec__bindgen_ty_1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct CInfoPBRec__bindgen_ty_1 {
    pub(crate) ioNamePtr: *mut ::std::os::raw::c_void,
}

impl Default for CInfoPBRec__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for CInfoPBRec {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct GDevice {
    pub(crate) gdRect: Rect,
}
pub(crate) type GDHandle = *mut *mut GDevice;
pub(crate) type DialogPtr = *mut ::std::os::raw::c_void;

unsafe extern "C" {
    pub(crate) fn putc(arg1: i32, arg2: *mut FILE) -> i32;

    pub(crate) fn strlen(__s: *const i8) -> u64;
    pub(crate) fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    pub(crate) fn strcat(__s1: *mut i8, __s2: *const i8) -> *mut i8;
    pub(crate) fn strcpy(__dst: *mut i8, __src: *const i8) -> *mut i8;
    pub(crate) fn sprintf(arg1: *mut i8, arg2: *const i8, ...) -> i32;
    pub(crate) fn fopen(__filename: *const i8, __mode: *const i8) -> *mut FILE;
    pub(crate) fn fprintf(arg1: *mut FILE, arg2: *const i8, ...) -> i32;
    pub(crate) fn fscanf(arg1: *mut FILE, arg2: *const i8, ...) -> i32;
    pub(crate) fn feof(arg1: *mut FILE) -> i32;
    pub(crate) fn fclose(arg1: *mut FILE) -> i32;
    pub(crate) fn getc(arg1: *mut FILE) -> i32;
}

pub fn convert_to_c_char_ptr(s: &str) -> *mut i8 {
    let c_string = CString::new(s).unwrap();
    c_string.into_raw()
}
