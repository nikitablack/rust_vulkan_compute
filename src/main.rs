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

    type M = nalgebra::OMatrix<f32, nalgebra::Dynamic, nalgebra::Dynamic>;
    let mut mat_a = M::from_element(constants::N, constants::N, 0.0f32);
    let mut mat_b = M::from_element(constants::N, constants::N, 0.0f32);

    for row in 0..constants::N {
        for col in 0..constants::N {
            let value = rng.gen_range(0.0f32..1.0f32);
            mat_a[(row, col)] = value;
            a[row * constants::N + col] = value;

            let value = rng.gen_range(0.0f32..1.0f32);
            mat_b[(row, col)] = value;
            b[row * constants::N + col] = value;
        }
    }

    for _ in 0..10 {
        // cpu matrix
        let start = std::time::Instant::now();

        let mut mat_c = M::from_element(constants::N, constants::N, 0.0f32);
        mat_a.mul_to(&mat_b, &mut mat_c);

        let duration = start.elapsed();

        println!("CPU time {}\n", duration.as_millis());

        // gpu matrix
        let start = std::time::Instant::now();

        let result = match vulkan_data.multiply(&a, &b) {
            Ok(data) => data,
            Err(msg) => {
                log::error!("{}", msg);
                panic!("{}", msg);
            }
        };

        let duration = start.elapsed();

        println!("Vulkan time + copy time {}\n", duration.as_millis());

        for row in 0..constants::N {
            for col in 0..constants::N {
                assert!(fuzzy_compare(
                    result[constants::N * row + col],
                    mat_c[(row, col)]
                ));
            }
        }
    }

    vulkan_data.clean();
}

fn fuzzy_compare(a: f32, b: f32) -> bool {
    const EPSILON: f32 = 0.01f32;
    return (a - b).abs() < EPSILON;
}
