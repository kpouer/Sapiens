#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::shared::{CGrafPtr, Rect};
pub(crate) type __int64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TPrStatus {
    pub(crate) dummy: ::std::os::raw::c_char,
}
pub(crate) const gestaltFindFolderAttr: u32 = 1718578276;
pub(crate) const times: u32 = 0;
pub(crate) const sampledSynth: u32 = 5;
pub(crate) const initMono: u32 = 128;

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
    pub(crate) txFont: ::std::os::raw::c_short,
    pub(crate) txFace: ::std::os::raw::c_short,
    pub(crate) txMode: ::std::os::raw::c_short,
    pub(crate) txSize: ::std::os::raw::c_short,
    pub(crate) lineHeight: ::std::os::raw::c_short,
    pub(crate) fontAscent: ::std::os::raw::c_short,
    pub(crate) fontDescent: ::std::os::raw::c_short,
    pub(crate) nLines: ::std::os::raw::c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct TMTask {
    pub(crate) qLink: *mut ::std::os::raw::c_void,
    pub(crate) qType: ::std::os::raw::c_short,
    pub(crate) tmAddr: *mut ::std::os::raw::c_void,
    pub(crate) tmCount: ::std::os::raw::c_long,
    pub(crate) tmWakeUp: ::std::os::raw::c_long,
    pub(crate) tmReserved: ::std::os::raw::c_long,
}
impl Default for TMTask {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
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
    pub(crate) bJDocLoop: ::std::os::raw::c_short,
}

pub(crate) type FILE = __sFILE;
pub(crate) const charCodeMask: u32 = 255;
pub(crate) const cmdKey: u32 = 256;
pub(crate) const ctrlItem: u32 = 16;
pub(crate) const bold: u32 = 1;
pub(crate) const editText: u32 = 4;

pub(crate) const watchCursor: u32 = 0;
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
    pub(crate) _base: *mut ::std::os::raw::c_uchar,
    pub(crate) _size: ::std::os::raw::c_int,
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
pub(crate) type ModalFilterUPP = *mut ::std::os::raw::c_void;
pub(crate) const patOr: u32 = 1;
pub(crate) const srcCopy: u32 = 0;
pub(crate) type THPrint = *mut *mut TPrint;

pub(crate) const DRVR: u32 = 1146246738;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct DateTimeRec {
    pub(crate) year: ::std::os::raw::c_short,
    pub(crate) month: ::std::os::raw::c_short,
    pub(crate) day: ::std::os::raw::c_short,
    pub(crate) hour: ::std::os::raw::c_short,
    pub(crate) minute: ::std::os::raw::c_short,
    pub(crate) second: ::std::os::raw::c_short,
    pub(crate) dayOfWeek: ::std::os::raw::c_short,
}
pub(crate) type fpos_t = __darwin_off_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct __sFILE {
    pub(crate) _p: *mut ::std::os::raw::c_uchar,
    pub(crate) _r: ::std::os::raw::c_int,
    pub(crate) _w: ::std::os::raw::c_int,
    pub(crate) _flags: ::std::os::raw::c_short,
    pub(crate) _file: ::std::os::raw::c_short,
    pub(crate) _bf: __sbuf,
    pub(crate) _lbfsize: ::std::os::raw::c_int,
    pub(crate) _cookie: *mut ::std::os::raw::c_void,
    pub(crate) _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub(crate) _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            __n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub(crate) _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub(crate) _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            __n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
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
    pub(crate) fn putc(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;

    pub(crate) fn GetMainDevice() -> GDHandle;
    pub(crate) fn RetraceRect(d: DialogPtr, r: *mut Rect);
    pub(crate) fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
    pub(crate) fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub(crate) fn strcat(
        __s1: *mut ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub(crate) fn strcpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub(crate) fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
    pub(crate) fn sprintf(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub(crate) fn stricmp(
        c1: *const ::std::os::raw::c_char,
        c2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub(crate) fn printf(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    pub(crate) fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __nitems: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
    pub(crate) static mut version: ::std::os::raw::c_short;

    pub(crate) fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub(crate) fn fprintf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub(crate) fn fscanf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub(crate) fn feof(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub(crate) fn fclose(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub(crate) fn getc(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub(crate) fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __nitems: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
    pub(crate) fn remove(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub(crate) fn FixeChemin(
        pathBase: ::std::os::raw::c_long,
        path: *mut ::std::os::raw::c_char,
    ) -> OSErr;

    pub(crate) static mut Ecran: CGrafPtr;

}
