use ash::vk;

pub fn create_descriptor_pool(device: &ash::Device) -> Result<vk::DescriptorPool, String> {
    log::info!("creating descriptor pool");

    let pool_size = vk::DescriptorPoolSize::builder()
        .ty(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(3)
        .build();

    let sizes = [pool_size];
    let create_info = vk::DescriptorPoolCreateInfo::builder()
        .max_sets(1)
        .pool_sizes(&sizes)
        .build();

    let pool = unsafe {
        device
            .create_descriptor_pool(&create_info, None)
            .map_err(|_| String::from("failed to create descriptor pool"))?
    };

    Ok(pool)
}
