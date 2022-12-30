use crate::vulkan::MemBuffer;
use ash::vk;

pub fn copy_data_to_buffer(
    device: &ash::Device,
    mem_buffer: &MemBuffer,
    offset: vk::DeviceSize,
    data: &[f32],
) -> Result<(), String> {
    let size = (data.len() * std::mem::size_of::<f32>()) as vk::DeviceSize;

    assert!(size <= mem_buffer.size);

    let mapped_data_ptr = unsafe {
        device
            .map_memory(
                mem_buffer.device_memory,
                offset,
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
        device.unmap_memory(mem_buffer.device_memory);
    }

    Ok(())
}
