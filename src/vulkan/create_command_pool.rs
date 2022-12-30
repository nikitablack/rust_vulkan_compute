use ash::vk;

pub fn create_command_pool(
    device: &ash::Device,
    queue_family: u32,
) -> Result<vk::CommandPool, String> {
    log::info!("creating command pool");

    let create_info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::TRANSIENT)
        .queue_family_index(queue_family)
        .build();

    let command_pool = unsafe {
        device
            .create_command_pool(&create_info, None)
            .map_err(|_| String::from("failed to create command pool"))?
    };

    Ok(command_pool)
}
