//! rebuilds shaders


use std::env;

use rust_gpu_shaders::recompile_shaders;
//pub const SHADER_FOLDER: &str = "shaders/";
use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main(){
    let Ok(out_dir) = env::current_dir() else {panic!("out dir doesn't exist??")};
    println!("recompiling shaders from: {:#?}", env::current_dir());
    let recompile_results = recompile_shaders(out_dir.to_str().unwrap().to_owned() + "/assets");

    println!("shader recompile results: ");
    println!("{:#?}", recompile_results);
}