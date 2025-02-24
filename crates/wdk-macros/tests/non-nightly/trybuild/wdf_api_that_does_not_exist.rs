// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

#![no_main]
use wdk_sys::*;

#[cfg(feature = "wdf")]
#[export_name = "DriverEntry"]// WDF expects a symbol with the name DriverEntry
pub extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    unsafe {
        wdk_macros::call_unsafe_wdf_function_binding!(
            WdfApiThatDoesNotExist,
            driver as PDRIVER_OBJECT,
        )
    }
}
