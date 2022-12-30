use ash::vk;

use super::VulkanData;

pub fn begin_command_buffer(
    vulkan_data: &VulkanData,
    command_buffer: vk::CommandBuffer,
) -> Result<(), String> {
    let begin_info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
        .build();

    unsafe {
        vulkan_data
            .device
            .begin_command_buffer(command_buffer, &begin_info)
            .map_err(|_| String::from("failed to begin command buffer"))?;
    }

    Ok(())
}
