use ash::vk;

use crate::constants;

pub fn create_pipeline(
    device: &ash::Device,
    shader_module: vk::ShaderModule,
    pipeline_layout: vk::PipelineLayout,
) -> Result<vk::Pipeline, String> {
    log::info!("creating pipeline");

    let shader_entry_name = std::ffi::CString::new("main").unwrap();

    let map_entry_0 = vk::SpecializationMapEntry::builder()
        .constant_id(0)
        .offset(0)
        .size(std::mem::size_of::<u32>())
        .build();

    let map_entry_1 = vk::SpecializationMapEntry::builder()
        .constant_id(1)
        .offset(std::mem::size_of::<u32>() as u32)
        .size(std::mem::size_of::<u32>())
        .build();

    let map_entry_2 = vk::SpecializationMapEntry::builder()
        .constant_id(2)
        .offset(2 * std::mem::size_of::<u32>() as u32)
        .size(std::mem::size_of::<u32>())
        .build();

    let map_entries = [map_entry_0, map_entry_1, map_entry_2];

    let data_0 = constants::WORKGROUP_SIZE.to_ne_bytes();
    let data_1 = constants::WORKGROUP_SIZE.to_ne_bytes();
    let data_2 = 1u32.to_ne_bytes();

    let data = [data_0, data_1, data_2].concat();

    let specialization_info = vk::SpecializationInfo::builder()
        .map_entries(&map_entries)
        .data(&data)
        .build();

    let pipeline_shader_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::COMPUTE)
        .module(shader_module)
        .name(&shader_entry_name)
        .specialization_info(&specialization_info)
        .build();

    let pipeline_create_info = vk::ComputePipelineCreateInfo::builder()
        .flags(vk::PipelineCreateFlags::empty())
        .stage(pipeline_shader_stage)
        .layout(pipeline_layout)
        .build();

    let pipelines = unsafe {
        device
            .create_compute_pipelines(vk::PipelineCache::null(), &[pipeline_create_info], None)
            .map_err(|_| String::from("failed to create pipelines"))?
    };

    Ok(pipelines[0])
}
