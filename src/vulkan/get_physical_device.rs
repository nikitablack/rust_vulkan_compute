use ash::vk;

fn check_required_device_extensions(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    required_extensions: &Vec<&std::ffi::CStr>,
) -> Result<(), String> {
    log::info!(
        "checking required device extensions: {:?}",
        required_extensions
    );

    let supported_device_extensions =
        match unsafe { instance.enumerate_device_extension_properties(physical_device) } {
            Ok(props) => props,
            Err(_) => {
                return Err(String::from(
                    "failed to enumerate instance extension properies",
                ))
            }
        };

    let mut supported_device_extensions_set = std::collections::HashSet::new();
    for vk::ExtensionProperties { extension_name, .. } in &supported_device_extensions {
        supported_device_extensions_set
            .insert(unsafe { std::ffi::CStr::from_ptr(extension_name.as_ptr()) });
    }

    for extension_name in required_extensions {
        if !supported_device_extensions_set.contains(extension_name) {
            return Err(format!(
                "device extension {:?} is not supported",
                extension_name
            ));
        }
    }

    Ok(())
}

fn check_device_suitability(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    required_extensions: &Vec<&std::ffi::CStr>,
    properties: &vk::PhysicalDeviceProperties,
) -> Result<(), String> {
    // api version
    log::info!(
        "supported api version: {}.{}.{}",
        vk::api_version_major(properties.api_version),
        vk::api_version_minor(properties.api_version),
        vk::api_version_patch(properties.api_version)
    );

    if vk::api_version_major(properties.api_version) < 1
        && vk::api_version_minor(properties.api_version) < 2
    {
        return Err(String::from(
            "the device does not support API version 1.2.0",
        ));
    }

    // features
    let features = unsafe { instance.get_physical_device_features(physical_device) };

    // needed for printf
    if features.fragment_stores_and_atomics == 0 {
        return Err(String::from(
            "the device does not support fragment stores and atomics",
        ));
    }

    // needed for printf
    if features.vertex_pipeline_stores_and_atomics == 0 {
        return Err(String::from(
            "the device does not support vertex pipeline stores and atomics",
        ));
    }

    // properties
    let properties = unsafe { instance.get_physical_device_properties(physical_device) };

    // needed for timestamp queries
    if properties.limits.timestamp_compute_and_graphics == 0 {
        return Err(String::from(
            "the device does not support timestamp compute and graphics",
        ));
    }

    // needed for timestamp queries
    if properties.limits.timestamp_period == 0.0f32 {
        return Err(String::from(
            "the device does not support timestamp queries",
        ));
    }

    check_required_device_extensions(instance, physical_device, required_extensions)?;

    Ok(())
}

pub fn get_physical_device<'a>(
    instance: &ash::Instance,
    required_device_extensions: &Vec<&'a std::ffi::CStr>,
) -> Result<vk::PhysicalDevice, String> {
    log::info!("enumerating physical devices");

    let devices = match unsafe { instance.enumerate_physical_devices() } {
        Ok(devices) => devices,
        Err(_) => return Err(String::from("failed to enumerate physical devices")),
    };

    log::info!("available physical devices: ");
    for &physical_device in &devices {
        let properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) };
        log::info!("{:?}", device_name);
    }

    for physical_device in devices {
        let properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) };

        log::info!("checking physical device: {:?}", device_name);

        if let Err(msg) = check_device_suitability(
            instance,
            physical_device,
            required_device_extensions,
            &properties,
        ) {
            log::warn!("{:?}: {}", device_name, msg);
            continue;
        }

        log::info!("selected physical device {:?}", device_name);

        return Ok(physical_device);
    }

    Err(String::from("failed to find suitable device"))
}
