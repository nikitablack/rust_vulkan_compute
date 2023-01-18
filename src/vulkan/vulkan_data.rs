use ash::vk;

use crate::constants;

use super::{begin_command_buffer, update_descriptor_set};

pub struct VulkanData {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
    pub queue_family: u32,
    pub device: ash::Device,
    pub debug_utils: super::DebugUtils,
    pub queue: vk::Queue,
    pub mem_buffer_a: super::MemBuffer,
    pub mem_buffer_b: super::MemBuffer,
    pub mem_buffer_c: super::MemBuffer,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
    pub command_pool: vk::CommandPool,
    pub descriptor_pool: vk::DescriptorPool,
    pub query_pool: vk::QueryPool,
}

impl VulkanData {
    pub fn new<'a, 'b>(
        required_instance_extensions: &Vec<&'a std::ffi::CStr>,
        required_device_extensions: &Vec<&'b std::ffi::CStr>,
    ) -> Result<Self, String> {
        let entry = super::create_entry();
        super::check_instance_version(&entry)?;
        super::check_required_instance_extensions(&entry, required_instance_extensions)?;
        let instance = super::create_instance(&entry, required_instance_extensions)?;
        let physical_device = super::get_physical_device(&instance, &required_device_extensions)?;

        let physical_device_properties =
            super::get_physical_device_properties(&instance, physical_device);

        let queue_family = super::get_queue_family(&instance, physical_device)?;

        let device = super::create_logical_device(
            &instance,
            physical_device,
            queue_family,
            &required_device_extensions,
        )?;

        let debug_utils = super::DebugUtils::new(&entry, &instance, device.handle());

        let queue = super::get_queue(&device, queue_family);

        // buffer a
        let mem_buffer_a = super::create_mem_buffer(
            &instance,
            physical_device,
            &device,
            constants::DATA_SIZE as vk::DeviceSize,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        debug_utils.set_name(mem_buffer_a.buffer, "matrix A buffer");
        debug_utils.set_name(mem_buffer_a.device_memory, "matrix A device memory");

        // buffer b
        let mem_buffer_b = super::create_mem_buffer(
            &instance,
            physical_device,
            &device,
            constants::DATA_SIZE as vk::DeviceSize,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        debug_utils.set_name(mem_buffer_b.buffer, "matrix B buffer");
        debug_utils.set_name(mem_buffer_b.device_memory, "matrix B device memory");

        // buffer c
        let mem_buffer_c = super::create_mem_buffer(
            &instance,
            physical_device,
            &device,
            constants::DATA_SIZE as vk::DeviceSize,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        debug_utils.set_name(mem_buffer_c.buffer, "matrix C buffer");
        debug_utils.set_name(mem_buffer_c.device_memory, "matrix C device memory");

        // shader module
        let shader_module =
            super::create_shader_module(&device, std::path::Path::new("shaders/shader.comp.spv"))?;

        debug_utils.set_name(shader_module, "shader module");

        // descriptor set layout
        let descriptor_set_layout = super::create_descriptor_set_layout(&device)?;

        debug_utils.set_name(descriptor_set_layout, "decriptor set layout");

        // pipeline layout
        let pipeline_layout = super::create_pipeline_layout(&device, descriptor_set_layout)?;

        debug_utils.set_name(pipeline_layout, "pipeline layout");

        // pipeline
        let pipeline = super::create_pipeline(&device, shader_module, pipeline_layout)?;

        debug_utils.set_name(pipeline, "pipeline");

        // destroy shader module
        unsafe {
            device.destroy_shader_module(shader_module, None);
        }

        // command pool
        let command_pool = super::create_command_pool(&device, queue_family)?;

        debug_utils.set_name(command_pool, "command pool");

        // descriptor pool
        let descriptor_pool = super::create_descriptor_pool(&device)?;

        debug_utils.set_name(descriptor_pool, "descriptor pool");

        // query pool
        let query_pool = super::create_query_pool(&device)?;

        debug_utils.set_name(query_pool, "query pool");

        Ok(VulkanData {
            entry,
            instance,
            physical_device,
            physical_device_properties,
            queue_family,
            device,
            debug_utils,
            queue,
            mem_buffer_a,
            mem_buffer_b,
            mem_buffer_c,
            descriptor_set_layout,
            pipeline_layout,
            pipeline,
            command_pool,
            descriptor_pool,
            query_pool,
        })
    }

    pub fn clean(self) {
        log::info!("cleaning vulkan data");

        unsafe {
            self.device.destroy_query_pool(self.query_pool, None);

            self.device
                .destroy_descriptor_pool(self.descriptor_pool, None);

            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_pipeline(self.pipeline, None);

            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);

            self.device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.device.destroy_buffer(self.mem_buffer_a.buffer, None);

            self.device
                .free_memory(self.mem_buffer_a.device_memory, None);

            self.device.destroy_buffer(self.mem_buffer_b.buffer, None);

            self.device
                .free_memory(self.mem_buffer_b.device_memory, None);

            self.device.destroy_buffer(self.mem_buffer_c.buffer, None);

            self.device
                .free_memory(self.mem_buffer_c.device_memory, None);

            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }

    pub fn multiply(&self, a: &[f32], b: &[f32]) -> Result<Vec<f32>, String> {
        super::copy_data_to_buffer(self, &self.mem_buffer_a, a)?;
        super::copy_data_to_buffer(self, &self.mem_buffer_b, b)?;

        let start = std::time::Instant::now();

        let command_buffer = super::allocate_command_buffer(self)?;
        let descriptor_set = super::allocate_descriptor_set(self)?;

        begin_command_buffer(self, command_buffer)?;
        update_descriptor_set(self, descriptor_set);

        unsafe {
            self.device.cmd_push_constants(
                command_buffer,
                self.pipeline_layout,
                vk::ShaderStageFlags::COMPUTE,
                0,
                &(constants::N as u32).to_ne_bytes(),
            );

            self.device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline_layout,
                0,
                &[descriptor_set],
                &[],
            );

            self.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline,
            );

            const GROUP_COUNT: u32 = (constants::N as u32) / constants::WORKGROUP_SIZE;

            assert!(
                GROUP_COUNT
                    <= self
                        .physical_device_properties
                        .limits
                        .max_compute_work_group_count[0]
            );
            assert!(
                GROUP_COUNT
                    <= self
                        .physical_device_properties
                        .limits
                        .max_compute_work_group_count[1]
            );

            self.device
                .cmd_reset_query_pool(command_buffer, self.query_pool, 0, 2);

            self.device.cmd_write_timestamp(
                command_buffer,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                self.query_pool,
                0,
            );

            self.device
                .cmd_dispatch(command_buffer, GROUP_COUNT, GROUP_COUNT, 1);

            self.device.cmd_write_timestamp(
                command_buffer,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                self.query_pool,
                1,
            );

            self.device
                .end_command_buffer(command_buffer)
                .map_err(|_| String::from("failed to end command buffer"))?
        }

        super::submit(self, command_buffer)?;

        unsafe {
            // wait until the GPU is done with all work
            self.device
                .device_wait_idle()
                .map_err(|_| String::from("failed to wait device idle"))?;

            let mut query_data = [0u64; 2];

            self.device
                .get_query_pool_results(
                    self.query_pool,
                    0,
                    2,
                    &mut query_data,
                    vk::QueryResultFlags::TYPE_64,
                )
                .map_err(|_| String::from("failed to get query pool results"))?;

            let timestamp_start = query_data[0];
            let timestamp_end = query_data[1];

            println!(
                "GPU timestamp {}",
                ((timestamp_end - timestamp_start) as f32)
                    * self.physical_device_properties.limits.timestamp_period
                    / 1000000.0f32
            );

            // free command buffer
            self.device
                .free_command_buffers(self.command_pool, &[command_buffer]);

            // reset command pool
            self.device
                .reset_command_pool(
                    self.command_pool,
                    vk::CommandPoolResetFlags::RELEASE_RESOURCES,
                )
                .map_err(|_| String::from("failed to reset command pool"))?;

            // reset descriptor pool
            self.device
                .reset_descriptor_pool(self.descriptor_pool, vk::DescriptorPoolResetFlags::empty())
                .map_err(|_| String::from("failed to reset descriptor pool"))?;

            let duration = start.elapsed();

            println!("vulkan time {}", duration.as_millis());

            // read the data back
            let data = super::read_data_from_buffer(
                self,
                &self.mem_buffer_c,
                constants::DATA_SIZE as vk::DeviceSize,
            )?;

            Ok(data)
        }
    }
}
