use ash::extensions::ext;
use ash::vk;

pub struct DebugUtils {
    device_handle: vk::Device,
    debug_utils_loader: ext::DebugUtils,
}

impl DebugUtils {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance, device_handle: vk::Device) -> Self {
        let debug_utils_loader = ext::DebugUtils::new(&entry, &instance);

        Self {
            device_handle,
            debug_utils_loader,
        }
    }
}

impl DebugUtils {
    pub fn set_name<T: vk::Handle>(&self, object_handle: T, object_name: &str) {
        let name_cstr = std::ffi::CString::new(object_name).expect("wrong string parameter");

        let name_info = vk::DebugUtilsObjectNameInfoEXT::builder()
            .object_type(T::TYPE)
            .object_handle(object_handle.as_raw())
            .object_name(&name_cstr);

        let _ = unsafe {
            self.debug_utils_loader
                .debug_utils_set_object_name(self.device_handle, &name_info)
        };
    }
}
