use ash::vk;

use super::VulkanData;

pub fn allocate_command_buffer(vulkan_data: &VulkanData) -> Result<vk::CommandBuffer, String> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(vulkan_data.command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1)
        .build();

    let command_buffers = unsafe {
        vulkan_data
            .device
            .allocate_command_buffers(&allocate_info)
            .map_err(|_| String::from("failed to allocate command buffer"))?
    };

    Ok(command_buffers[0])
}
