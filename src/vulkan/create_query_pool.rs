use ash::vk;

pub fn create_query_pool(device: &ash::Device) -> Result<vk::QueryPool, String> {
    let create_info = vk::QueryPoolCreateInfo::builder()
        .query_type(vk::QueryType::TIMESTAMP)
        .query_count(2)
        .build();

    let query_pool = unsafe {
        device
            .create_query_pool(&create_info, None)
            .map_err(|_| String::from("failed to create query pool"))?
    };

    Ok(query_pool)
}
