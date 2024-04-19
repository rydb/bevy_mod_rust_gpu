//! rebuilds shaders


use std::env;

//pub const SHADER_FOLDER: &str = "shaders/";
use spirv_builder::{MetadataPrintout, SpirvBuilder};

pub fn recompile_shaders(out_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    //let simplest_shader = SHADER_FOLDER.to_owned() + "simplest_shader";
    let post_processing = "assets/shaders/rust_gpu_shaders/post_processing";
    SpirvBuilder::new(post_processing, "spirv-unknown-vulkan1.2")
        .print_metadata(MetadataPrintout::Full)
        .extra_arg(format!("OUT_DIR={:#}", out_dir))
        .build()?;
    Ok(())
    
}