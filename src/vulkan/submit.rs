use ash::vk;

use super::VulkanData;

pub fn submit(vulkan_data: &VulkanData, command_buffer: vk::CommandBuffer) -> Result<(), String> {
    let cmd_buffers = [command_buffer];
    let submit_info = vk::SubmitInfo::builder()
        .command_buffers(&cmd_buffers)
        .build();

    unsafe {
        vulkan_data
            .device
            .queue_submit(vulkan_data.queue, &[submit_info], vk::Fence::null())
            .map_err(|_| String::from("failed to submit graphics command buffer"))?
    }

    Ok(())
}
