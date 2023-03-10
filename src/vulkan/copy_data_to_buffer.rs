use crate::vulkan::MemBuffer;
use ash::vk;

use super::VulkanData;

pub fn copy_data_to_buffer(
    vulkan_data: &VulkanData,
    mem_buffer: &MemBuffer,
    data: &[f32],
) -> Result<(), String> {
    let size = (data.len() * std::mem::size_of::<f32>()) as vk::DeviceSize;
    assert!(size <= mem_buffer.size);

    // create staging buffer
    let staging_mem_buffer = super::create_mem_buffer(
        &vulkan_data.instance,
        vulkan_data.physical_device,
        &vulkan_data.device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    )?;

    // copy data to staging buffer
    let mapped_data_ptr = unsafe {
        vulkan_data
            .device
            .map_memory(
                staging_mem_buffer.device_memory,
                0,
                size,
                vk::MemoryMapFlags::empty(),
            )
            .map_err(|_| String::from("failed to map buffer memory"))?
    };

    let mut data_slice = unsafe {
        ash::util::Align::new(
            mapped_data_ptr,
            std::mem::align_of::<f32>() as vk::DeviceSize,
            size,
        )
    };

    data_slice.copy_from_slice(data);

    unsafe {
        vulkan_data
            .device
            .unmap_memory(staging_mem_buffer.device_memory);
    }

    // allocate command buffer
    let command_buffer = super::allocate_command_buffer(vulkan_data)?;

    // begin command buffer
    super::begin_command_buffer(vulkan_data, command_buffer)?;

    // copy data to device local buffer
    let buffer_copy = vk::BufferCopy::builder().size(size).build();

    unsafe {
        vulkan_data.device.cmd_copy_buffer(
            command_buffer,
            staging_mem_buffer.buffer,
            mem_buffer.buffer,
            &[buffer_copy],
        );

        vulkan_data
            .device
            .end_command_buffer(command_buffer)
            .map_err(|_| String::from("failed to end command buffer"))?
    }

    // submit
    super::submit(vulkan_data, command_buffer)?;

    // wait
    unsafe {
        // wait until the GPU is done with all work
        vulkan_data
            .device
            .device_wait_idle()
            .map_err(|_| String::from("failed to wait device idle"))?;
    }

    // clean
    unsafe {
        // destroy buffer
        vulkan_data
            .device
            .destroy_buffer(staging_mem_buffer.buffer, None);

        // free memory
        vulkan_data
            .device
            .free_memory(staging_mem_buffer.device_memory, None);

        // free command buffer
        vulkan_data
            .device
            .free_command_buffers(vulkan_data.command_pool, &[command_buffer]);

        // reset command pool
        vulkan_data
            .device
            .reset_command_pool(
                vulkan_data.command_pool,
                vk::CommandPoolResetFlags::RELEASE_RESOURCES,
            )
            .map_err(|_| String::from("failed to reset command pool"))?;
    }

    Ok(())
}
