use std::{env::var, path::PathBuf};
use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Dx12Compiler::Dxc, Gles3MinorVersion::Automatic, Instance, InstanceDescriptor, InstanceFlags, PowerPreference, Queue, RenderPipelineDescriptor, RequestAdapterOptions, SurfaceTarget, WindowHandle
};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Document, Element, Gpu, HtmlCanvasElement, HtmlElement, Navigator, Node, Window};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(start)]
pub async fn run() {
    let window = if let Some(win) = create_window_handle() { win } else { panic!("FATAL: Window must exist for execution to continue") };
    let canvas = get_canvas_context(&window);


    let out_dir = var("OUT_DIR").unwrap();
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::PRIMARY,
        flags: InstanceFlags::DEBUG,
        dx12_shader_compiler: Dxc { dxil_path: Some(PathBuf::from(&out_dir).join("dxc")), dxc_path: Some(PathBuf::from(&out_dir).join("dxc")) },
        gles_minor_version: Automatic
    });

    let surface_target = SurfaceTarget::Canvas(canvas);
    let surface = instance.create_surface(surface_target).unwrap(); // Handle this better than with unwrap

    let adapter: Adapter = instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface)
    }).await.unwrap_throw();

    let  (device, queue) = adapter.request_device(&DeviceDescriptor {
        required_features: wgpu::Features::all_webgpu_mask(),
        required_limits: wgpu::Limits::default(),
        memory_hints: wgpu::MemoryHints::Performance,
        label: None
    }, None).await.unwrap_throw();
}

fn get_canvas_context(window: &Window) -> HtmlCanvasElement {
    let document = window.document().expect("FATAL: Unable to attain a reference to the document object associated with the global window");
    let canvas = document.get_element_by_id("gpu-canvas").unwrap_or_else(|| {
        let element = document.create_element("canvas").expect("FATAL: Error occured while creating a canvas element");
        element.set_id("gpu-canvas");
        document.body().expect("FATAL: HTML document is not valid, unable to attain a handle to the body element, or body does not exist")
                       .append_child(&element)
                       .expect("FATAL: Unable to append the canvas element to the document, aborting");
        element
    }).dyn_into::<HtmlCanvasElement>().expect("Unable to cast the Element into the HtmlCanvasElement");
    canvas
}

fn create_window_handle() -> Option<Window> {
    web_sys::window()
}