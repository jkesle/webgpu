use std::path::PathBuf;

use fs_extra::dir::CopyOptions;

fn main() -> () {
    let out = std::env::var("OUT_DIR").unwrap();
    let target = PathBuf::from(&out);
    let target_architecture = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let project_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dx_path = format!("{project_root}\\lib\\dxc\\{target_architecture}\\dxc");

    println!("target: {}", &dx_path);

    fs_extra::dir::copy(
        dx_path,
        target,
        &CopyOptions {
            skip_exist: true,
            overwrite: true,
            depth: 0,
            buffer_size: 64000,
            copy_inside: false,
            content_only: false,
        },
    )
    .expect("Cannot copy dx12 compiler to output directory");
}
