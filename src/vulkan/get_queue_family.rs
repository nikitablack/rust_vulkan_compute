use ash::vk;

pub fn get_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<u32, String> {
    log::info!("getting queue family");

    let props = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    for (ind, p) in props.iter().enumerate() {
        if p.queue_count > 0 && p.queue_flags.contains(vk::QueueFlags::COMPUTE) {
            log::info!("selected queue family: {}", ind);
            return Ok(ind as u32);
        }
    }

    Err(String::from(
        "failed to find graphics queue with present support",
    ))
}
