use ash::vk;
use std::io::Read;

pub fn create_shader_module(
    device: &ash::Device,
    path: &std::path::Path,
) -> Result<vk::ShaderModule, String> {
    log::info!("creating shader module");

    let mut file =
        std::fs::File::open(path).map_err(|_| format!("failed to open file {:?}", path))?;

    let mut spirv_u8 = Vec::new();
    let _ = file
        .read_to_end(&mut spirv_u8)
        .map_err(|_| format!("failed to read file {:?}", path))?;

    let spirv_u32 = ash::util::read_spv(&mut std::io::Cursor::new(&spirv_u8))
        .map_err(|_| format!("failed to read spirv {:?}", path))?;

    let create_info = vk::ShaderModuleCreateInfo::builder()
        .code(&spirv_u32)
        .build();

    let shader_module = unsafe {
        device
            .create_shader_module(&create_info, None)
            .map_err(|_| format!("failed to create shader module {:?}", path))?
    };

    Ok(shader_module)
}
