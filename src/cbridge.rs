#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub(crate) type __int64_t = ::std::os::raw::c_longlong;

pub(crate) type OSErr = i16;
pub(crate) type MenuHandle = *mut ::std::os::raw::c_void;
pub(crate) const everyEvent: u32 = 65535;
pub(crate) const helvetica: u32 = 2;
pub(crate) const optionKey: u32 = 2048;

pub(crate) const teJustLeft: u32 = 0;
pub(crate) const geneva: u32 = 0;

pub(crate) type WindowPtr = *mut ::std::os::raw::c_void;

pub(crate) const charCodeMask: u32 = 255;
pub(crate) const cmdKey: u32 = 256;
pub(crate) const shiftKey: u32 = 512;
pub(crate) const controlKey: u32 = 4096;
pub(crate) const activateEvt: u32 = 8;
pub(crate) const app4Evt: u32 = 55;
pub(crate) const bold: u32 = 1;

pub(crate) const watchCursor: u32 = 0;
pub(crate) const keyDown: u32 = 3;
// Codes d'événement et résultats de `FindWindow` (Inside Macintosh).
pub(crate) const mouseDown: u32 = 1;
pub(crate) const mouseUp: u32 = 2;
pub(crate) const inSysWindow: u32 = 2;
pub(crate) const inContent: u32 = 3;
pub(crate) const inDrag: u32 = 4;
pub(crate) const blackColor: u32 = 33;
pub(crate) const whiteColor: u32 = 30;

pub(crate) const patOr: u32 = 1;
pub(crate) const srcCopy: u32 = 0;

pub(crate) fn strlen<T: Copy + Default + PartialEq>(buf: &[T]) -> usize {
    buf.iter()
        .position(|&c| c == T::default())
        .unwrap_or(buf.len())
}
