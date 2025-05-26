use std::process::Command;

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
        "default.vert",
        "default.frag",
        "transparent.vert",
        "transparent.frag",
    ];

    println!("cargo:rerun-if-changed=/dev/null");

    shaders
        .iter()
        .for_each(|s| {
            let shader_name = (*s).split(".").collect::<Vec<_>>();
            Command::new("glslc")
                .arg(format!("src/shaders/{}", *s))
                .arg("-o")
                .arg(format!("src/shaders/{}_{}.spv", shader_name[0], shader_name[1]))
                .status()
                .unwrap_or_else(|e| panic!("Error compiling shaders {}", e));
        });
}