//! Rust packed GxAPI interface
#![allow(dead_code)]

use libloading::{Library, Symbol};
use std::ffi::{c_void,c_char};

// use crate::{
//     gx::{gx_enum::*, gx_handle::*, gx_struct::*},
//     utils::status::convert_to_gx_status,
// };

use crate::gx::{
    gx_enum::*, gx_handle::*, gx_struct::*,
};

fn convert_to_gx_status(status_code: i32) -> GX_STATUS_LIST {
    match status_code {
        0 => GX_STATUS_LIST::GX_STATUS_SUCCESS,
        -1 => GX_STATUS_LIST::GX_STATUS_ERROR,
        -2 => GX_STATUS_LIST::GX_STATUS_NOT_FOUND_TL,
        -3 => GX_STATUS_LIST::GX_STATUS_NOT_FOUND_DEVICE,
        -4 => GX_STATUS_LIST::GX_STATUS_OFFLINE,
        -5 => GX_STATUS_LIST::GX_STATUS_INVALID_PARAMETER,
        -6 => GX_STATUS_LIST::GX_STATUS_INVALID_HANDLE,
        -7 => GX_STATUS_LIST::GX_STATUS_INVALID_CALL,
        -8 => GX_STATUS_LIST::GX_STATUS_INVALID_ACCESS,
        -9 => GX_STATUS_LIST::GX_STATUS_NEED_MORE_BUFFER,
        -10 => GX_STATUS_LIST::GX_STATUS_ERROR_TYPE,
        -11 => GX_STATUS_LIST::GX_STATUS_OUT_OF_RANGE,
        -12 => GX_STATUS_LIST::GX_STATUS_NOT_IMPLEMENTED,
        -13 => GX_STATUS_LIST::GX_STATUS_NOT_INIT_API,
        -14 => GX_STATUS_LIST::GX_STATUS_TIMEOUT,
        _ => GX_STATUS_LIST::GX_STATUS_ERROR, // Default error if unknown status code
    }
}

pub type GXCaptureCallBack = extern "C" fn(pFrameData: *mut GX_FRAME_CALLBACK_PARAM);

// Define a custom error type
#[derive(Debug)]
pub enum CameraError {
    LibraryError(libloading::Error),
    OperationError(String),
}

impl From<libloading::Error> for CameraError {
    fn from(err: libloading::Error) -> Self {
        CameraError::LibraryError(err)
    }
}
pub trait GXInterface {
    unsafe fn new(library_path: &str) -> Result<Self, libloading::Error>
    where
        Self: Sized;

    // Lib
    unsafe fn gx_init_lib(&self) -> Result<i32, libloading::Error>;
    unsafe fn gx_close_lib(&self) -> Result<(), libloading::Error>;

    // Device
    unsafe fn gx_update_device_list(
        &self,
        device_num: *mut u32,
        timeout: u32,
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_get_all_device_base_info(
        &self,
        p_device_info: *mut GX_DEVICE_BASE_INFO,
        p_buffer_size: *mut usize,
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_open_device_by_index(
        &self,
        index: u32,
        device: *mut GX_DEV_HANDLE,
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_close_device(&self, device: GX_DEV_HANDLE) -> Result<i32, libloading::Error>;
    
    // Command
    unsafe fn gx_send_command(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
    ) -> Result<(), CameraError>;

    // Image
    unsafe fn gx_get_image(
        &self,
        device: GX_DEV_HANDLE,
        p_frame_data: *mut GX_FRAME_DATA,
        timeout: i32,
    ) -> Result<(), CameraError>;

    // Getter and Setter
    unsafe fn gx_get_int(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_value: *mut i64,
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_set_int(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_value: i64,
    ) -> Result<i32, libloading::Error>;


    unsafe fn gx_get_float(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_value: *mut f64,
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_set_float(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_value: f64,
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_get_enum(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_value: *mut i64,
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_set_enum(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_value: i64,
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_get_bool(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        value: *mut bool,
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_set_bool(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        value: bool,
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_get_string_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize, // 使用 usize 来表示 size_t
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_get_string_max_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize, // 使用 usize 来表示 size_t
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_get_string(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        content: *mut c_char,
        size: *mut usize,
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_set_string(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        content: *const c_char,
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_get_buffer_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize, // Using usize to represent size_t
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_get_buffer(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        buffer: *mut u8,
        size: *mut usize, // Using usize to represent size_t
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_set_buffer(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        buffer: *const u8,
        size: usize, // Using usize to represent size_t
    ) -> Result<i32, libloading::Error>;

    unsafe fn gx_get_int_range(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_range: *mut GX_INT_RANGE, // Assume GX_INT_RANGE is defined somewhere
    ) -> Result<i32, libloading::Error>;
    unsafe fn gx_get_float_range(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_range: *mut GX_FLOAT_RANGE, // Assume GX_FLOAT_RANGE is defined somewhere
    ) -> Result<i32, libloading::Error>;


    // Callback
    unsafe fn gx_register_capture_callback(
        &self,
        device: *mut c_void,
        callback: GXCaptureCallBack,
    ) -> Result<(), CameraError>;
    unsafe fn gx_unregister_capture_callback(&self, device: *mut c_void)
        -> Result<(), CameraError>;
}

pub struct GXInstance {
    pub lib: Library,
}

impl GXInterface for GXInstance {
    /// Create a new GXInterface instance
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     // Other Operations
    ///
    /// }
    /// ```
    unsafe fn new(library_path: &str) -> Result<Self, libloading::Error> {
        let lib = Library::new(library_path)?;
        Ok(GXInstance { lib })
    }

    /// Initialize the library
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     gx.gx_init_lib().expect("Failed to initialize library");
    ///
    ///     // Other Operations
    ///
    /// }
    /// ```
    unsafe fn gx_init_lib(&self) -> Result<i32, libloading::Error> {
        let gx_init_lib: Symbol<unsafe extern "C" fn() -> i32> = self.lib.get(b"GXInitLib")?;
        Ok(gx_init_lib())
    }

    /// Close library
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     gx.gx_init_lib().expect("Failed to initialize library");
    ///
    ///     // Other Operations
    ///
    ///     gx.gx_close_lib().expect("Failed to close library");
    ///
    /// }
    /// ```
    unsafe fn gx_close_lib(&self) -> Result<(), libloading::Error> {
        let gx_close_lib: Symbol<unsafe extern "C" fn() -> i32> = self.lib.get(b"GXCloseLib")?;
        gx_close_lib();
        Ok(())
    }

    /// Update the device list
    ///
    /// # Examples
    ///
    /// ```
    /// use gxi_hako::gx::{
    ///     gx_interface::*
    /// };
    ///
    /// unsafe {
    ///
    ///    let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
    ///
    ///     gx.gx_init_lib().expect("Failed to initialize library");
    ///
    ///     let mut device_num = 0;
    ///     gx.gx_update_device_list(&mut device_num, 1000)
    ///         .expect("Failed to update device list");
    ///
    ///     // Other Operations
    ///
    ///     gx.gx_close_lib().expect("Failed to close library");
    ///
    /// }
    /// ```
    unsafe fn gx_update_device_list(
        &self,
        device_num: *mut u32,
        timeout: u32,
    ) -> Result<i32, libloading::Error> {
        let gx_update_device_list: Symbol<
            unsafe extern "C" fn(device_num: *mut u32, timeout: u32) -> i32,
        > = self.lib.get(b"GXUpdateDeviceList")?;
        Ok(gx_update_device_list(device_num, timeout))
    }

    /// Get all device base info
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    ///

    unsafe fn gx_get_all_device_base_info(
        &self,
        p_device_info: *mut GX_DEVICE_BASE_INFO,
        p_buffer_size: *mut usize,
    ) -> Result<i32, libloading::Error> {
        let gx_get_all_device_base_info: Symbol<
            unsafe extern "C" fn(
                p_device_info: *mut GX_DEVICE_BASE_INFO,
                p_buffer_size: *mut usize,
            ) -> i32,
        > = self.lib.get(b"GXGetAllDeviceBaseInfo")?;
        println!(
            "p_device_info: {:?}, p_buffer_size: {:?}",
            p_device_info, p_buffer_size
        );
        Ok(gx_get_all_device_base_info(p_device_info, p_buffer_size))
    }

    /// Open device by index
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    ///
    unsafe fn gx_open_device_by_index(
        &self,
        index: u32,
        device: *mut GX_DEV_HANDLE,
    ) -> Result<i32, libloading::Error> {
        let gx_open_device_by_index: Symbol<
            unsafe extern "C" fn(index: u32, device: *mut GX_DEV_HANDLE) -> i32,
        > = self.lib.get(b"GXOpenDeviceByIndex")?;
        Ok(gx_open_device_by_index(index, device))
    }

    /// Send command to device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_send_command(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
    ) -> Result<(), CameraError> {
        let gx_send_command: Symbol<unsafe extern "C" fn(GX_DEV_HANDLE, GX_FEATURE_ID) -> i32> =
            self.lib.get(b"GXSendCommand")?;

        let status_code = gx_send_command(device, feature_id);
        let status = convert_to_gx_status(status_code);
        match status {
            GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(()),
            _ => Err(CameraError::OperationError(format!(
                "GXSendCommand failed with status: {:?}",
                status
            ))),
        }
    }

    /// Get image from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_get_image(
        &self,
        device: GX_DEV_HANDLE,
        p_frame_data: *mut GX_FRAME_DATA,
        timeout: i32,
    ) -> Result<(), CameraError> {
        let gx_get_image: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                p_frame_data: *mut GX_FRAME_DATA,
                timeout: i32,
            ) -> i32,
        > = self.lib.get(b"GXGetImage")?;
        println!("p_frame_data: {:?}", p_frame_data);
        println!("frame_data: {:?}", *p_frame_data);
        let status_code = gx_get_image(device, p_frame_data, timeout);
        let status = convert_to_gx_status(status_code);
        match status {
            GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(()),
            _ => Err(CameraError::OperationError(format!(
                "Failed to get image with status: {:?}",
                status
            ))),
        }
    }

    /// Close device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_close_device(&self, device: GX_DEV_HANDLE) -> Result<i32, libloading::Error> {
        let gx_close_device: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE) -> i32> =
            self.lib.get(b"GXCloseDevice")?;
        Ok(gx_close_device(device))
    }

    /// Get int value from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_get_int(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_value: *mut i64,
    ) -> Result<i32, libloading::Error> {
        let gx_get_int: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                int_value: *mut i64,
            ) -> i32,
        > = self.lib.get(b"GXGetInt")?;
        println!("int_value: {:?}", int_value);
        Ok(gx_get_int(device, feature_id, int_value))
    }

    /// Set int value for device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_set_int(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_value:  i64,
    ) -> Result<i32, libloading::Error> {
        let gx_set_int: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                int_value: i64,
            ) -> i32,
        > = self.lib.get(b"GXSetInt")?;
        println!("int_value: {:?}", int_value);
        Ok(gx_set_int(device, feature_id, int_value))
    }


    /// Get float value from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_get_float(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_value: *mut f64,
    ) -> Result<i32, libloading::Error> {
        let gx_get_float: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                float_value: *mut f64,
            ) -> i32,
        > = self.lib.get(b"GXGetFloat")?;
        println!("int_value: {:?}", float_value);
        Ok(gx_get_float(device, feature_id, float_value))
    }

    /// Set float value for device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_set_float(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_value: f64,
    ) -> Result<i32, libloading::Error> {
        let gx_set_float: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                float_value: f64,
            ) -> i32,
        > = self.lib.get(b"GXSetFloat")?;
        println!("int_value: {:?}", float_value);
        Ok(gx_set_float(device, feature_id, float_value))
    }

    /// Get enum value from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_get_enum(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_value: *mut i64,
    ) -> Result<i32, libloading::Error> {
        let gx_get_enum: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                enum_value: *mut i64,
            ) -> i32,
        > = self.lib.get(b"GXGetEnum")?;
        println!("enum_value: {:?}", enum_value);
        Ok(gx_get_enum(device, feature_id, enum_value))
    }


    /// Set enum value for device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_set_enum(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        enum_value: i64,
    ) -> Result<i32, libloading::Error> {
        let gx_set_enum: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                enum_value: i64,
            ) -> i32,
        > = self.lib.get(b"GXSetEnum")?;
        println!("enum_value: {:?}", enum_value);
        Ok(gx_set_enum(device, feature_id, enum_value))
    }


    /// Get bool value from device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_get_bool(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        bool_value: *mut bool,
    ) -> Result<i32, libloading::Error> {
        let gx_get_bool: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                bool_value: *mut bool,
            ) -> i32,
        > = self.lib.get(b"GXGetBool")?;
        println!("bool_value: {:?}", bool_value);
        Ok(gx_get_bool(device, feature_id, bool_value))
    }

    /// Set bool value for device
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_set_bool(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        bool_value: bool,
    ) -> Result<i32, libloading::Error> {
        let gx_set_bool: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                bool_value: bool,
            ) -> i32,
        > = self.lib.get(b"GXSetBool")?;
        println!("bool_value: {:?}", bool_value);
        Ok(gx_set_bool(device, feature_id, bool_value))
    }


    /// Get the length of the string value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    unsafe fn gx_get_string_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize, // Using usize to represent size_t
    ) -> Result<i32, libloading::Error> {
        let gx_get_string_length: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                size: *mut usize,
            ) -> i32,
        > = self.lib.get(b"GXGetStringLength")?;
        println!("size: {:?}", size);
        Ok(gx_get_string_length(device, feature_id, size))
    }

    /// Get the maximum length of the string value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    unsafe fn gx_get_string_max_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize, // Using usize to represent size_t
    ) -> Result<i32, libloading::Error> {
        let gx_get_string_max_length: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                size: *mut usize,
            ) -> i32,
        > = self.lib.get(b"GXGetStringMaxLength")?;
        println!("size: {:?}", size);
        Ok(gx_get_string_max_length(device, feature_id, size))
    }

    /// Get the string value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    unsafe fn gx_get_string(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        content: *mut c_char,
        size: *mut usize,
    ) -> Result<i32, libloading::Error> {
        let gx_get_string: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                content: *mut c_char,
                size: *mut usize,
            ) -> i32,
        > = self.lib.get(b"GXGetString")?;
        println!("content: {:?}, size: {:?}", content, size);
        Ok(gx_get_string(device, feature_id, content, size))
    }   


    /// Set the string value for the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    unsafe fn gx_set_string(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        content: *const c_char,
    ) -> Result<i32, libloading::Error> {
        let gx_set_string: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                content: *const c_char,
            ) -> i32,
        > = self.lib.get(b"GXSetString")?;
        println!("content: {:?}", content);
        Ok(gx_set_string(device, feature_id, content))
    }

    /// Get the length of the block data from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    /// 
    unsafe fn gx_get_buffer_length(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        size: *mut usize,
    ) -> Result<i32, libloading::Error> {
        let gx_get_buffer_length: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                size: *mut usize,
            ) -> i32,
        > = self.lib.get(b"GXGetBufferLength")?;
        println!("size: {:?}", size);
        Ok(gx_get_buffer_length(device, feature_id, size))
    }

    /// Get the block data from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    unsafe fn gx_get_buffer(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        buffer: *mut u8,
        size: *mut usize,
    ) -> Result<i32, libloading::Error> {
        let gx_get_buffer: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                buffer: *mut u8,
                size: *mut usize,
            ) -> i32,
        > = self.lib.get(b"GXGetBuffer")?;
        println!("buffer: {:?}, size: {:?}", buffer, size);
        Ok(gx_get_buffer(device, feature_id, buffer, size))
    }
    
    /// Set the block data for the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    /// 
    unsafe fn gx_set_buffer(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        buffer: *const u8,
        size: usize,
    ) -> Result<i32, libloading::Error> {
        let gx_set_buffer: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                buffer: *const u8,
                size: usize,
            ) -> i32,
        > = self.lib.get(b"GXSetBuffer")?;
        println!("buffer: {:?}, size: {:?}", buffer, size);
        Ok(gx_set_buffer(device, feature_id, buffer, size))
    }
    
    /// Get the range of an integer type value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    unsafe fn gx_get_int_range(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        int_range: *mut GX_INT_RANGE,
    ) -> Result<i32, libloading::Error> {
        let gx_get_int_range: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                int_range: *mut GX_INT_RANGE,
            ) -> i32,
        > = self.lib.get(b"GXGetIntRange")?;
        println!("int_range: {:?}", int_range);
        Ok(gx_get_int_range(device, feature_id, int_range))
    }
    
    /// Get the range of a float type value from the device
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::gx::gx_interface::GXInterface;
    /// ```
    unsafe fn gx_get_float_range(
        &self,
        device: GX_DEV_HANDLE,
        feature_id: GX_FEATURE_ID,
        float_range: *mut GX_FLOAT_RANGE,
    ) -> Result<i32, libloading::Error> {
        let gx_get_float_range: Symbol<
            unsafe extern "C" fn(
                device: GX_DEV_HANDLE,
                feature_id: GX_FEATURE_ID,
                float_range: *mut GX_FLOAT_RANGE,
            ) -> i32,
        > = self.lib.get(b"GXGetFloatRange")?;
        println!("float_range: {:?}", float_range);
        Ok(gx_get_float_range(device, feature_id, float_range))
    }
    

    /// Register capture callback
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_register_capture_callback(
        &self,
        device: *mut c_void,
        callback: GXCaptureCallBack,
    ) -> Result<(), CameraError> {
        let gx_register_capture_callback: Symbol<
            unsafe extern "C" fn(
                device: *mut c_void,
                user_param: *mut c_void,
                callback: GXCaptureCallBack,
            ) -> i32,
        > = self.lib.get(b"GXRegisterCaptureCallback")?;
        let status_code = gx_register_capture_callback(device, std::ptr::null_mut(), callback);
        let status = convert_to_gx_status(status_code);
        match status {
            GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(()),
            _ => Err(CameraError::OperationError(format!(
                "Failed to register_callback with status: {:?}",
                status
            ))),
        }
    }

    /// Unregister capture callback
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use crate::gx::gx_interface::GXInterface;
    ///
    /// ```
    unsafe fn gx_unregister_capture_callback(
        &self,
        device: *mut c_void,
    ) -> Result<(), CameraError> {
        let gx_unregister_capture_callback: Symbol<
            unsafe extern "C" fn(device: *mut c_void) -> i32,
        > = self.lib.get(b"GXUnregisterCaptureCallback")?;
        let status_code = gx_unregister_capture_callback(device);
        let status = convert_to_gx_status(status_code);
        match status {
            GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(()),
            _ => Err(CameraError::OperationError(format!(
                "Failed to unregister_callback with status: {:?}",
                status
            ))),
        }
    }
}

// 相关定义如下
// pub type GX_DEV_HANDLE = *mut c_void;
// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct GX_FRAME_DATA {
//     pub status: u32,            // Image acquisition status
//     pub frame_id: u64,          // Frame ID
//     pub p_img_buf: *mut c_void, // Pointer to the image buffer
//     pub img_size: i32,          // Size of the image buffer, adjusted to i32 to match C definition
//     pub width: i32,             // Image width, adjusted to i32 to match C definition
//     pub height: i32,            // Image height, adjusted to i32 to match C definition
//     pub pixel_format: i32,      // Pixel format, adjusted to i32 to match C definition
//     pub timestamp: u64,         // Timestamp of the frame
//     pub offset_x: i32,          // X offset of the image
//     pub offset_y: i32,          // Y offset of the image
//     pub reserved: [i32; 1],     // Reserved, array of 1 i32 to match C definition
// }
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_STATUS_LIST {
//     GX_STATUS_SUCCESS = 0,
//     GX_STATUS_ERROR = -1,
//     GX_STATUS_NOT_FOUND_TL = -2,
//     GX_STATUS_NOT_FOUND_DEVICE = -3,
//     GX_STATUS_OFFLINE = -4,
//     GX_STATUS_INVALID_PARAMETER = -5,
//     GX_STATUS_INVALID_HANDLE = -6,
//     GX_STATUS_INVALID_CALL = -7,
//     GX_STATUS_INVALID_ACCESS = -8,
//     GX_STATUS_NEED_MORE_BUFFER = -9,
//     GX_STATUS_ERROR_TYPE = -10,
//     GX_STATUS_OUT_OF_RANGE = -11,
//     GX_STATUS_NOT_IMPLEMENTED = -12,
//     GX_STATUS_NOT_INIT_API = -13,
//     GX_STATUS_TIMEOUT = -14,
// }
