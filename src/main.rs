mod constants;
mod matrix;
mod vulkan;

use matrix::Matrix;

use vulkan::VulkanData;

use rand::Rng;

fn main() {
    // logger
    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![simplelog::TermLogger::new(
        simplelog::LevelFilter::Off,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )];
    if let Ok(file) = std::fs::File::create("log.txt") {
        loggers.push(simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            file,
        ));
    }

    let _ = simplelog::CombinedLogger::init(loggers);

    let mut a = Matrix::new(4);
    a.fill(1.0f32);

    let mut b = Matrix::new(4);
    b.fill(2.0f32);

    let c = a.mul(&b);

    println!("{:?}", c);

    let device_extensions = vec![
        ash::vk::KhrShaderNonSemanticInfoFn::name(),
        ash::vk::KhrShaderClockFn::name(),
    ];
    let instance_extensions = vec![ash::extensions::ext::DebugUtils::name()];

    let vulkan_data = match VulkanData::new(&instance_extensions, &device_extensions) {
        Ok(data) => data,
        Err(msg) => {
            log::error!("{}", msg);
            panic!("{}", msg);
        }
    };

    let mut rng = rand::thread_rng();

    let mut a = vec![0.0f32; constants::N * constants::N];
    let mut b = vec![0.0f32; constants::N * constants::N];

    a.fill_with(|| rng.gen_range(0.0f32..1.0f32));
    b.fill_with(|| rng.gen_range(0.0f32..1.0f32));

    for _ in 0..10 {
        let start = std::time::Instant::now();

        let _result = match vulkan_data.multiply(&a, &b) {
            Ok(data) => data,
            Err(msg) => {
                log::error!("{}", msg);
                panic!("{}", msg);
            }
        };

        let duration = start.elapsed();

        println!("total time {}\n", duration.as_millis());
    }

    vulkan_data.clean();
}
