//! rebuilds shaders


use std::{env, fs};

use rust_gpu_shaders::recompile_shaders;
//pub const SHADER_FOLDER: &str = "shaders/";
use spirv_builder::{MetadataPrintout, SpirvBuilder};

pub fn recompile_all_shaders(shaders_root_crate: &str, spirv_out_folder: &str) {
    let shader_crates = fs::read_dir(shaders_root_crate).unwrap()
    .filter_map(|x| x.ok())
    .filter(|x| x.path().is_dir())
    .filter(|x| x.path().components().last().unwrap().as_os_str() != "src")
    ;
    for path in shader_crates.filter(|x| x.path().is_dir()) {
        let shader_name = path.file_name();
        println!("compiling shader: {:#?}", shader_name);

        let compile_results = recompile_shaders(shader_name.to_str().unwrap(), &spirv_out_folder);
        println!("compile results: {:#?}", compile_results);

    }
}

fn main(){
    let Ok(out_dir_check) = env::current_dir() else {panic!("out dir doesn't exist??")};

    let out_dir = out_dir_check.to_str().unwrap().to_owned();
    let spirv_out_folder = out_dir + &"/assets/shaders";


    //let paths = fs::read_dir("./").unwrap();

    //FIXME: get proper crate directory at some point. 
    let this_crate_path = spirv_out_folder.clone() + &"/rust_gpu_shaders";

    //recompile_all_shaders(&this_crate_path, &spirv_out_folder);

    println!("recompiling shaders from: {:#?}", this_crate_path);
    let recompile_results = recompile_shaders("rave_shader", &spirv_out_folder);

    println!("{:#?}", recompile_results);
}