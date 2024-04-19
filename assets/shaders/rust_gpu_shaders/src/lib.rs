//! rebuilds shaders


//pub const SHADER_FOLDER: &str = "shaders/";
use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let simplest_shader = SHADER_FOLDER.to_owned() + "simplest_shader";
    //let simplest_shader = "/home/ry/Projects/bevy_mod_rust_gpu/assets/shaders/rust_gpu_shaders/post_processing";
    
    let post_processing = "post_processing";
    //let simplest_shader = concat!(env!("CARGO_MANIFEST_DIR"), "simplest_shader");
    //println!("BUILDING SHADERS NOW WOOOOOOOO");
    //println!("COME THE FUCK OOOOON");
    SpirvBuilder::new(post_processing, "spirv-unknown-vulkan1.2")
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    Ok(())
}