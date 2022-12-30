use ash::vk;

use crate::constants;

pub struct MemBuffer {
    pub buffer: vk::Buffer,
    pub device_memory: vk::DeviceMemory,
    pub size: vk::DeviceSize,
}

pub fn create_mem_buffer(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &ash::Device,
    memory_flags: vk::MemoryPropertyFlags,
) -> Result<MemBuffer, String> {
    log::info!("getting mem buffer");

    let size = constants::DATA_SIZE as vk::DeviceSize;

    let buffer = create_buffer(device, size, vk::BufferUsageFlags::STORAGE_BUFFER)?;

    let memory_type =
        find_buffer_memory_type(instance, physical_device, device, buffer, memory_flags)?;

    let device_memory = create_device_memory(device, buffer, memory_type)?;

    unsafe {
        device
            .bind_buffer_memory(buffer, device_memory, 0)
            .map_err(|_| String::from("failed to bind buffer and memory"))?;
    }

    Ok(MemBuffer {
        buffer,
        device_memory,
        size,
    })
}

fn create_buffer(
    device: &ash::Device,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
) -> Result<vk::Buffer, String> {
    let buffer_create_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .build();

    let buffer = unsafe {
        device
            .create_buffer(&buffer_create_info, None)
            .map_err(|_| String::from("failed to create buffer"))?
    };

    Ok(buffer)
}

fn find_buffer_memory_type(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &ash::Device,
    buffer: vk::Buffer,
    memory_flags: vk::MemoryPropertyFlags,
) -> Result<u32, String> {
    let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

    let memory_property_index = get_supported_memory_property_index(
        instance,
        physical_device,
        memory_requirements.memory_type_bits,
        memory_flags,
    )?;

    Ok(memory_property_index)
}

fn get_supported_memory_property_index(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    supported_memory_type_bits: u32,
    desired_memory_flags: vk::MemoryPropertyFlags,
) -> Result<u32, String> {
    let memory_properties =
        unsafe { instance.get_physical_device_memory_properties(physical_device) };

    for i in 0..memory_properties.memory_type_count {
        let memory_type_supported = (supported_memory_type_bits & (1u32 << i)) > 0;

        if memory_type_supported
            && memory_properties.memory_types[i as usize]
                .property_flags
                .contains(desired_memory_flags)
        {
            return Ok(i);
        }
    }

    Err(String::from("failed to find memory property index"))
}

fn create_device_memory(
    device: &ash::Device,
    buffer: vk::Buffer,
    memory_type_index: u32,
) -> Result<vk::DeviceMemory, String> {
    let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

    let allocate_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(memory_requirements.size)
        .memory_type_index(memory_type_index)
        .build();

    let device_memory = unsafe {
        device
            .allocate_memory(&allocate_info, None)
            .map_err(|_| String::from("failed to allocate device memory"))?
    };

    Ok(device_memory)
}
