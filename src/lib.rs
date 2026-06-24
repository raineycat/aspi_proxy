use std::{ffi::c_void, path::PathBuf, ptr::slice_from_raw_parts};

use ftail::Ftail;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};

lazy_static! {
    static ref ASPI_LIB: Library = load_original();
}

type HINSTANCE = *const c_void;

type tfnInit = extern "cdecl" fn() -> i32;
type tfnGetDeviceNum = extern "cdecl" fn(*mut i32) -> i32;
type tfnOpenDevice = extern "cdecl" fn(i32) -> i32;
type tfnCloseDevice = extern "cdecl" fn(i32) -> i32;
type tfnGetInquiry = extern "cdecl" fn(i32, *mut c_void, u32) -> i32;
type tfnDataSize = extern "cdecl" fn(i32, *mut u32, *mut u32) -> i32;
type tfnReadData = extern "cdecl" fn(i32, *mut c_void, *mut u32) -> i32;
type tfnWriteData = extern "cdecl" fn(i32, *mut c_void, *mut u32) -> i32;
type tfnSendVendorCommand = extern "cdecl" fn(i32, *mut c_void) -> i32;

#[unsafe(no_mangle)]
extern "system" fn DllMain(hInstDll: HINSTANCE, fdwReason: u32, lpvReserved: *const c_void) {
    if fdwReason == 1 {
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

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_Init() -> i32 {
    log::debug!("Init");
    let func = ASPI_LIB.get::<tfnInit>(b"fxASPI_Init").unwrap();
    let result = func();
    log::debug!("fxASPI_Init() -> {result}");
    return result;
}

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_GetDeviceNum(count: *mut i32) -> i32 {
    log::debug!("GetDeviceNum");
    let func = ASPI_LIB
        .get::<tfnGetDeviceNum>(b"fxASPI_GetDeviceNum")
        .unwrap();
    let result = func(count);
    let devCount = *count;
    log::debug!("fxASPI_GetDeviceNum(&{devCount}) -> {result}");
    return result;
}

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_OpenDevice(idx: i32) -> i32 {
    log::debug!("OpenDevice");
    let func = ASPI_LIB.get::<tfnOpenDevice>(b"fxASPI_OpenDevice").unwrap();
    let result = func(idx);
    log::debug!("fxASPI_OpenDevice({idx}) -> {result}");
    return result;
}

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_CloseDevice(idx: i32) -> i32 {
    log::debug!("CloseDevice");
    let func = ASPI_LIB
        .get::<tfnCloseDevice>(b"fxASPI_CloseDevice")
        .unwrap();
    let result = func(idx);
    log::debug!("fxASPI_CloseDevice({idx}) -> {result}");
    return result;
}

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_GetInquiry(idx: i32, buf: *mut c_void, size: u32) -> i32 {
    log::debug!("GetInquiry");
    let func = ASPI_LIB.get::<tfnGetInquiry>(b"fxASPI_GetInquiry").unwrap();
    let result = func(idx, buf, size);
    log::debug!("fxASPI_OpenDevice({idx}, {buf:?}, {size}) -> {result}");
    let data = slice_from_raw_parts(buf as *const u8, size as usize);
    log::debug!("Inquiry: {data:02X?}");
    return result;
}

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_DataSize(idx: i32, pRead: *mut u32, pWrite: *mut u32) -> i32 {
    log::debug!("DataSize");
    let func = ASPI_LIB.get::<tfnDataSize>(b"fxASPI_DataSize").unwrap();
    let result = func(idx, pRead, pWrite);
    let readSize = if pRead.is_null() { 0 } else { *pRead };
    let writeSize = if pWrite.is_null() { 0 } else { *pWrite };
    log::debug!("fxASPI_DataSize({idx}, {readSize}, {writeSize}) -> {result}");
    return result;
}

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_ReadData(idx: i32, buffer: *mut c_void, size: *mut u32) -> i32 {
    log::debug!("ReadData");
    let func = ASPI_LIB.get::<tfnReadData>(b"fxASPI_ReadData").unwrap();
    let result = func(idx, buffer, size);
    let sizeVal = *size;
    log::debug!("fxASPI_ReadData({idx}, {buffer:?}, {sizeVal}) -> {result}");
    let data = slice_from_raw_parts(buffer as *const u8, *size as usize);
    log::debug!("Read: {data:02X?}");
    return result;
}

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_WriteData(idx: i32, buffer: *mut c_void, size: *mut u32) -> i32 {
    log::debug!("WriteData");
    let func = ASPI_LIB.get::<tfnWriteData>(b"fxASPI_WriteData").unwrap();
    let result = func(idx, buffer, size);
    let sizeVal = *size;
    log::debug!("fxASPI_WriteData({idx}, {buffer:?}, {sizeVal}) -> {result}");
    let data = slice_from_raw_parts(buffer as *const u8, *size as usize);
    log::debug!("Write: {data:02X?}");
    return result;
}

#[unsafe(no_mangle)]
unsafe extern "cdecl" fn fxASPI_SendVendorCommand(idx: i32, cmd: *mut c_void) -> i32 {
    log::debug!("SendVendorCommand");
    let func = ASPI_LIB
        .get::<tfnSendVendorCommand>(b"fxASPI_SendVendorCommand")
        .unwrap();
    let result = func(idx, cmd);
    log::debug!("fxASPI_ReadData({idx}, {cmd:?}) -> {result}");
    let data = slice_from_raw_parts(cmd as *const u8, 10);
    log::debug!("VendorCmd: {data:02X?}");
    return result;
}
