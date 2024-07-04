use std::process::{Command};

fn main() {
    compile_shaders();
}

fn compile_shaders() {
    Command::new("cmd.exe")
        .arg("/C")
        .arg(".\\src\\shaders\\compile_shaders.bat")
        .status()
        .unwrap_or_else(|e| panic!("Error compiling shaders {}", e));
}