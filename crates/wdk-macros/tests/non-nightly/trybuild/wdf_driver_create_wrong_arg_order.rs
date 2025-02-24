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
    let mut driver_config = WDF_DRIVER_CONFIG {
        Size: core::mem::size_of::<WDF_DRIVER_CONFIG>() as ULONG,
        ..WDF_DRIVER_CONFIG::default()
    };
    let driver_handle_output = WDF_NO_HANDLE as *mut WDFDRIVER;

    unsafe {
        wdk_macros::call_unsafe_wdf_function_binding!(
            WdfDriverCreate,
            driver as PDRIVER_OBJECT,
            registry_path,
            // The order of the next two arguements is swapped!
            &mut driver_config,
            WDF_NO_OBJECT_ATTRIBUTES,
            driver_handle_output,
        )
    }
}
