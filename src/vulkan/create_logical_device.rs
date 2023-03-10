use ash::vk;

pub fn create_logical_device<'a>(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
    device_extensions: &Vec<&'a std::ffi::CStr>,
) -> Result<ash::Device, String> {
    log::info!("creating logical device");

    let queue_indices = [queue_family];

    let mut queue_priorities = Vec::new();
    for _ in &queue_indices {
        queue_priorities.push(vec![1.0f32]);
    }

    let mut queue_create_infos = Vec::with_capacity(queue_indices.len());

    for (ind, &family_index) in queue_indices.iter().enumerate() {
        let info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(family_index)
            .queue_priorities(&queue_priorities[ind]);

        queue_create_infos.push(info.build());
    }

    let features = vk::PhysicalDeviceFeatures::builder()
        .shader_int64(true)
        .build();

    let mut shader_clock_features = vk::PhysicalDeviceShaderClockFeaturesKHR::builder()
        .shader_device_clock(true)
        .build();

    let mut features2 = vk::PhysicalDeviceFeatures2::builder()
        .features(features)
        .push_next(&mut shader_clock_features)
        .build();

    let device_extensions_raw = device_extensions
        .iter()
        .map(|&s| s.as_ptr())
        .collect::<Vec<*const std::os::raw::c_char>>();

    let create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(&device_extensions_raw)
        .push_next(&mut features2)
        .build();

    let device = unsafe {
        instance
            .create_device(physical_device, &create_info, None)
            .map_err(|_| String::from("failed to create device"))?
    };

    return Ok(device);
}
