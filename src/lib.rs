use std::{env::var, path::PathBuf};
use wgpu::{Instance, InstanceDescriptor, Backends, InstanceFlags, Dx12Compiler::Dxc, Gles3MinorVersion::Automatic, PowerPreference, RequestAdapterOptions};
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document, Element, HtmlElement, Node, Gpu};

#[wasm_bindgen(start)]
pub fn run() {
    let window: Window = web_sys::window().expect("no global window");
    let document: Document = window.document().expect("window should contain document object");
    let body: HtmlElement = document.body().expect("body element should be present on document");

    let gpu: Gpu = window.navigator().gpu();
    let adapter = gpu.request_adapter();

    let out_dir = var("OUT_DIR").unwrap();

    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::BROWSER_WEBGPU,
        flags: InstanceFlags::DEBUG,
        dx12_shader_compiler: Dxc { dxil_path: Some(PathBuf::from(&out_dir).join("dxc")), dxc_path: Some(PathBuf::from(&out_dir).join("dxc")) },
        gles_minor_version: Automatic
    });
}