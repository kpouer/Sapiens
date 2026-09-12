#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::shared::{Boolean, CGrafPtr, Rect};
pub type __int64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TPrStatus {
    pub dummy: ::std::os::raw::c_char,
}
pub const gestaltFindFolderAttr: u32 = 1718578276;
pub const times: u32 = 0;
pub const sampledSynth: u32 = 5;
pub const initMono: u32 = 128;

pub type OSErr = i16;
pub type MenuHandle = *mut ::std::os::raw::c_void;
pub const everyEvent: u32 = 65535;
pub const helvetica: u32 = 2;
pub const inMenuBar: u32 = 1;
pub const inContent: u32 = 3;
pub const inDrag: u32 = 4;
pub const inSysWindow: u32 = 2;
pub const itemDisable: u32 = 128;
pub const mouseDown: u32 = 1;
pub const optionKey: u32 = 2048;
pub const statText: u32 = 8;
pub const radCtrl: u32 = 20;
pub const userItem: u32 = 32;

pub const updateEvt: u32 = 4;

pub type TPPrPort = *mut ::std::os::raw::c_void;
pub const RSRC_TEXT_CARTE_ENREG: u32 = 2000;
pub const kOnSystemDisk: u32 = 32768;
pub const kPreferencesFolderType: u32 = 1886545254;
pub const kCreateFolder: u32 = 1;
pub type TMTaskPtr = *mut TMTask;
pub type QElemPtr = *mut ::std::os::raw::c_void;
pub type SndChannelPtr = *mut ::std::os::raw::c_void;
pub const teJustLeft: u32 = 0;
pub const geneva: u32 = 0;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEText {
    pub txFont: ::std::os::raw::c_short,
    pub txFace: ::std::os::raw::c_short,
    pub txMode: ::std::os::raw::c_short,
    pub txSize: ::std::os::raw::c_short,
    pub lineHeight: ::std::os::raw::c_short,
    pub fontAscent: ::std::os::raw::c_short,
    pub fontDescent: ::std::os::raw::c_short,
    pub nLines: ::std::os::raw::c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TMTask {
    pub qLink: *mut ::std::os::raw::c_void,
    pub qType: ::std::os::raw::c_short,
    pub tmAddr: *mut ::std::os::raw::c_void,
    pub tmCount: ::std::os::raw::c_long,
    pub tmWakeUp: ::std::os::raw::c_long,
    pub tmReserved: ::std::os::raw::c_long,
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
pub type ListHandle = *mut ::std::os::raw::c_void;
pub type NavObjectFilterUPP = *mut ::std::os::raw::c_void;
pub type NavObjectFilterProcPtr = *mut ::std::os::raw::c_void;
pub type NavEventUPP = *mut ::std::os::raw::c_void;
pub type NavEventProcPtr = *mut ::std::os::raw::c_void;
pub type WindowPtr = *mut ::std::os::raw::c_void;

pub type __darwin_off_t = __int64_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TPrint {
    pub prInfo: TPrint__bindgen_ty_1,
    pub prJob: TPrint__bindgen_ty_2,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TPrint__bindgen_ty_1 {
    pub rPage: Rect,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TPrint__bindgen_ty_2 {
    pub bJDocLoop: ::std::os::raw::c_short,
}

pub type FILE = __sFILE;
pub const charCodeMask: u32 = 255;
pub const courier: u32 = 1;
pub const noErr: u32 = 0;
pub const cmdKey: u32 = 256;
pub const ctrlItem: u32 = 16;
pub const activateEvt: u32 = 8;
pub const app4Evt: u32 = 55;
pub const bold: u32 = 1;
pub const editText: u32 = 4;

pub const watchCursor: u32 = 0;
pub const keyDown: u32 = 3;
pub const autoKey: u32 = 5;
pub const keyUp: u32 = 4;
pub const blackColor: u32 = 33;
pub const whiteColor: u32 = 30;

#[repr(C)]
#[derive(Debug)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
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
pub type ModalFilterUPP = *mut ::std::os::raw::c_void;
pub type FileFilterUPP = *mut ::std::os::raw::c_void;
pub const patOr: u32 = 1;
pub const srcCopy: u32 = 0;
pub type THPrint = *mut *mut TPrint;

pub const DRVR: u32 = 1146246738;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct DateTimeRec {
    pub year: ::std::os::raw::c_short,
    pub month: ::std::os::raw::c_short,
    pub day: ::std::os::raw::c_short,
    pub hour: ::std::os::raw::c_short,
    pub minute: ::std::os::raw::c_short,
    pub second: ::std::os::raw::c_short,
    pub dayOfWeek: ::std::os::raw::c_short,
}
pub type fpos_t = __darwin_off_t;
pub const DLOG: u32 = 1145851719;
pub const DITL: u32 = 1145656396;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            __n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            __n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
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
pub struct CInfoPBRec {
    pub hFileInfo: CInfoPBRec__bindgen_ty_1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CInfoPBRec__bindgen_ty_1 {
    pub ioNamePtr: *mut ::std::os::raw::c_void,
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
pub struct GDevice {
    pub gdRect: Rect,
}
pub type GDHandle = *mut *mut GDevice;
pub type DialogPtr = *mut ::std::os::raw::c_void;

pub type CInfoPBPtr = *mut CInfoPBRec;
unsafe extern "C" {
    pub fn putc(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;

    pub fn GetMainDevice() -> GDHandle;
    pub fn RetraceRect(d: DialogPtr, r: *mut Rect);
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strcat(
        __s1: *mut ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
    pub fn sprintf(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn stricmp(
        c1: *const ::std::os::raw::c_char,
        c2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn printf(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __nitems: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
    pub static mut version: ::std::os::raw::c_short;

    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn fprintf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn fscanf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn feof(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fclose(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getc(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __nitems: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
    pub fn memset(
        __b: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __len: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn remove(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn FixeChemin(pathBase: ::std::os::raw::c_long, path: *mut ::std::os::raw::c_char)
    -> OSErr;
    pub fn Collapse(format: *mut ::std::os::raw::c_char, ...);
    pub fn Confirme(format: *mut ::std::os::raw::c_char, ...) -> Boolean;

    pub static mut Ecran: CGrafPtr;

}
