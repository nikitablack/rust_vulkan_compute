pub const N: usize = 2048;
pub const WORKGROUP_SIZE: u32 = 16;
pub const DATA_SIZE: usize = N * N * std::mem::size_of::<f32>();
