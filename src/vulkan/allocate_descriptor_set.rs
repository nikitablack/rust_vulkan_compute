use ash::vk;

use super::VulkanData;

pub fn allocate_descriptor_set(vulkan_data: &VulkanData) -> Result<vk::DescriptorSet, String> {
    let layouts = [vulkan_data.descriptor_set_layout; 1];

    let alloc_info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(vulkan_data.descriptor_pool)
        .set_layouts(&layouts)
        .build();

    let descriptor_sets = unsafe {
        vulkan_data
            .device
            .allocate_descriptor_sets(&alloc_info)
            .map_err(|_| String::from("failed to allocate descriptor set"))?
    };

    let set = descriptor_sets[0];

    Ok(set)
}
