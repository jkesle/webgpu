use std::{env::var, path::PathBuf};
use wgpu::{Instance, InstanceDescriptor, Backends, InstanceFlags, Dx12Compiler::Dxc, Gles3MinorVersion::Automatic, PowerPreference, RequestAdapterOptions, Adapter, Device, SurfaceTarget, WindowHandle};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Document, Element, Gpu, HtmlCanvasElement, HtmlElement, Navigator, Node, Window};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(start)]
pub async fn run() {
    let window: Window = web_sys::window().expect("no global window");
    let document: Document = window.document().expect("window should contain document object");
    let canvas: HtmlCanvasElement = document.query_selector("#gpu-canvas").unwrap_throw().unwrap_throw().dyn_into::<HtmlCanvasElement>().expect("should be castable to HtmlCanvasElement");


    let out_dir = var("OUT_DIR").unwrap();
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::PRIMARY,
        flags: InstanceFlags::DEBUG,
        dx12_shader_compiler: Dxc { dxil_path: Some(PathBuf::from(&out_dir).join("dxc")), dxc_path: Some(PathBuf::from(&out_dir).join("dxc")) },
        gles_minor_version: Automatic
    });

    let surface_target = SurfaceTarget::Canvas(canvas);
    let surface = instance.create_surface(surface_target).unwrap(); // Handle this better than with unwrap

    let adapter: Adapter = instance.request_adapter(&RequestAdapterOptions { power_preference: PowerPreference::HighPerformance, force_fallback_adapter: false, compatible_surface: Some(&surface) }).await.unwrap_throw();

    todo!("Grab device from adapter");
}