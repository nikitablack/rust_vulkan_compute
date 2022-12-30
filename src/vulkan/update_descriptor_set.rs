use ash::vk;

use super::VulkanData;

pub fn update_descriptor_set(vulkan_data: &VulkanData, set: vk::DescriptorSet) {
    let info_a = vk::DescriptorBufferInfo::builder()
        .buffer(vulkan_data.mem_buffer_a.buffer)
        .offset(0)
        .range(vk::WHOLE_SIZE)
        .build();

    let info_b = vk::DescriptorBufferInfo::builder()
        .buffer(vulkan_data.mem_buffer_b.buffer)
        .offset(0)
        .range(vk::WHOLE_SIZE)
        .build();

    let info_c = vk::DescriptorBufferInfo::builder()
        .buffer(vulkan_data.mem_buffer_c.buffer)
        .offset(0)
        .range(vk::WHOLE_SIZE)
        .build();

    let infos = [info_a, info_b, info_c];
    let write_descriptor_set = vk::WriteDescriptorSet::builder()
        .dst_set(set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&infos)
        .build();

    unsafe {
        vulkan_data
            .device
            .update_descriptor_sets(&[write_descriptor_set], &[]);
    }
}
