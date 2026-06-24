use std::{ffi::c_void, io::Write, path::PathBuf, ptr::slice_from_raw_parts};

use base64::Engine;
use flate2::{Compression, write::ZlibEncoder};
use ftail::Ftail;
use lazy_static::lazy_static;
use libloading::Library;

lazy_static! {
    static ref ASPI_LIB: Library = load_original();
}

type HINSTANCE = *const c_void;

type FnInit = extern "C" fn() -> i32;
type FnGetDeviceNum = extern "C" fn(*mut i32) -> i32;
type FnOpenDevice = extern "C" fn(i32) -> i32;
type FnCloseDevice = extern "C" fn(i32) -> i32;
type FnGetInquiry = extern "C" fn(i32, *mut c_void, u32) -> i32;
type FnDataSize = extern "C" fn(i32, *mut u32, *mut u32) -> i32;
type FnReadData = extern "C" fn(i32, *mut c_void, *mut u32) -> i32;
type FnWriteData = extern "C" fn(i32, *mut c_void, *mut u32) -> i32;
type FnSendVendorCommand = extern "C" fn(i32, *mut c_void) -> i32;

#[unsafe(no_mangle)]
extern "system" fn DllMain(
    _dll_instance: HINSTANCE,
    reason_for_call: u32,
    _reserved: *const c_void,
) {
    if reason_for_call == 1 {
        let log_file = PathBuf::from("aspi_proxy.log");
        Ftail::new()
            .single_file(&log_file, false, log::LevelFilter::Debug)
            .init()
            .unwrap();
        log::info!("DLL attached!");
    }
}

fn load_original() -> Library {
    match unsafe { Library::new("o_fxASPI.dll") } {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to load original ASPI: {e}");
            panic!("wawa!");
        }
    }
}

fn dump_binary(buf: &[u8]) -> String {
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
    enc.write_all(buf).unwrap();
    let compressed = enc.finish().unwrap();
    let text = base64::prelude::BASE64_STANDARD.encode(&compressed);
    text
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_Init() -> i32 {
    unsafe {
        log::debug!("Init");
        let func = ASPI_LIB.get::<FnInit>(b"fxASPI_Init").unwrap();
        let result = func();
        log::debug!("fxASPI_Init() -> {result}");
        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_GetDeviceNum(count: *mut i32) -> i32 {
    unsafe {
        log::debug!("GetDeviceNum");
        let func = ASPI_LIB
            .get::<FnGetDeviceNum>(b"fxASPI_GetDeviceNum")
            .unwrap();
        let result = func(count);
        let count_val = *count;
        log::debug!("fxASPI_GetDeviceNum(&{count_val}) -> {result}");
        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_OpenDevice(idx: i32) -> i32 {
    unsafe {
        log::debug!("OpenDevice");
        let func = ASPI_LIB.get::<FnOpenDevice>(b"fxASPI_OpenDevice").unwrap();
        let result = func(idx);
        log::debug!("fxASPI_OpenDevice({idx}) -> {result}");
        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_CloseDevice(idx: i32) -> i32 {
    unsafe {
        log::debug!("CloseDevice");
        let func = ASPI_LIB
            .get::<FnCloseDevice>(b"fxASPI_CloseDevice")
            .unwrap();
        let result = func(idx);
        log::debug!("fxASPI_CloseDevice({idx}) -> {result}");
        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_GetInquiry(idx: i32, buf: *mut c_void, size: u32) -> i32 {
    unsafe {
        log::debug!("GetInquiry");
        let func = ASPI_LIB.get::<FnGetInquiry>(b"fxASPI_GetInquiry").unwrap();
        let result = func(idx, buf, size);
        log::debug!("fxASPI_OpenDevice({idx}, {buf:?}, {size}) -> {result}");
        let data = slice_from_raw_parts(buf as *const u8, size as usize);
        log::debug!("Inquiry: {}", dump_binary(&*data));
        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_DataSize(idx: i32, p_read: *mut u32, p_write: *mut u32) -> i32 {
    unsafe {
        log::debug!("DataSize");
        let func = ASPI_LIB.get::<FnDataSize>(b"fxASPI_DataSize").unwrap();
        let result = func(idx, p_read, p_write);
        let read_size = if p_read.is_null() { 0 } else { *p_read };
        let write_size = if p_write.is_null() { 0 } else { *p_write };
        log::debug!("fxASPI_DataSize({idx}, {read_size}, {write_size}) -> {result}");
        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_ReadData(idx: i32, buffer: *mut c_void, size: *mut u32) -> i32 {
    unsafe {
        log::debug!("ReadData");
        let func = ASPI_LIB.get::<FnReadData>(b"fxASPI_ReadData").unwrap();
        let result = func(idx, buffer, size);
        let size_val = *size;
        log::debug!("fxASPI_ReadData({idx}, {buffer:?}, {size_val}) -> {result}");
        let data = slice_from_raw_parts(buffer as *const u8, *size as usize);
        log::debug!("Read: {}", dump_binary(&*data));
        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_WriteData(idx: i32, buffer: *mut c_void, size: *mut u32) -> i32 {
    unsafe {
        log::debug!("WriteData");
        let func = ASPI_LIB.get::<FnWriteData>(b"fxASPI_WriteData").unwrap();
        let result = func(idx, buffer, size);
        let size_val = *size;
        log::debug!("fxASPI_WriteData({idx}, {buffer:?}, {size_val}) -> {result}");
        let data = slice_from_raw_parts(buffer as *const u8, *size as usize);
        log::debug!("Write: {}", dump_binary(&*data));
        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn fxASPI_SendVendorCommand(idx: i32, cmd: *mut c_void) -> i32 {
    unsafe {
        log::debug!("SendVendorCommand");
        let func = ASPI_LIB
            .get::<FnSendVendorCommand>(b"fxASPI_SendVendorCommand")
            .unwrap();
        let result = func(idx, cmd);
        log::debug!("fxASPI_ReadData({idx}, {cmd:?}) -> {result}");
        let data = slice_from_raw_parts(cmd as *const u8, 10);
        log::debug!("VendorCmd: {}", dump_binary(&*data));
        return result;
    }
}
