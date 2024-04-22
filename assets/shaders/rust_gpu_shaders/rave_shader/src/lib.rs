#![cfg_attr(target_arch = "spirv", no_std)]

use spirv_std::glam::{vec4, Vec4};
use spirv_std::spirv;

#[repr(C)]
pub struct MaterialColor {
    pub color: Vec4,
}

// #[spirv(vertex)]
// pub fn main_vs(
//     #[spirv(vertex_index)] vert_id: i32,
//     #[spirv(position, invariant)] out_pos: &mut Vec4,
// ) {
//     *out_pos = Vec4::default();
// }

#[spirv(fragment)]
pub fn main_fs( 
    #[spirv(uniform, descriptor_set = 2, binding = 0)] material_color: &MaterialColor,
    output: &mut Vec4
) {
    *output = material_color.color;
}