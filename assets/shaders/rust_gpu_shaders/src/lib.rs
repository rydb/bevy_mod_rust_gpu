//! rebuilds shaders


use std::env;

//pub const SHADER_FOLDER: &str = "shaders/";
use spirv_builder::{MetadataPrintout, SpirvBuilder};

pub fn recompile_shaders(shader_crate_name: &str, out_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    //let simplest_shader = SHADER_FOLDER.to_owned() + "simplest_shader";
    let post_processing = "assets/shaders/rust_gpu_shaders/".to_owned() + &shader_crate_name;
    SpirvBuilder::new(post_processing, "spirv-unknown-spv1.5")
        .print_metadata(MetadataPrintout::Full)
        .extra_arg(format!("OUT_DIR={:#}", out_dir))
        .build()?;
    Ok(())
}