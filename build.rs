use std::process::{Command};

fn main() {
    compile_shaders();
}

#[cfg(target_os = "windows")]
fn compile_shaders() {
    Command::new("cmd.exe")
        .arg("/C")
        .arg(".\\src\\shaders\\compile_shaders.bat")
        .status()
        .unwrap_or_else(|e| panic!("Error compiling shaders {}", e));
}

#[cfg(target_os = "macos")]
fn compile_shaders() {
    let shaders = vec![
        "shader.vert",
        "shader.frag"
    ];

    shaders
        .iter()
        .for_each(|s| {
            println!("cargo:rerun-if-changed=src/shaders/{}", *s);
            let shader_name: &str = (*s).split(".").collect::<Vec<_>>()[1];
            Command::new("glslc")
                .arg(format!("src/shaders/{}", *s))
                .arg("-o")
                .arg(format!("src/shaders/{}.spv", shader_name))
                .status()
                .unwrap_or_else(|e| panic!("Error compiling shaders {}", e));
        });
}