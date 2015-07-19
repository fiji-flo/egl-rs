extern crate gl_generator;
extern crate khronos_api;

use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest = Path::new(&out_dir);

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    // This generates bindsings for OpenGL ES v3.1
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gles2,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GL_XML,
                                    vec![],
                                    "3.1", "core", &mut file).unwrap();
}
