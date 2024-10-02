use std::{env::var, future::Future, path::PathBuf};
use wgpu::{
    Adapter, Backends, CreateSurfaceError, Device, DeviceDescriptor, Dx12Compiler::Dxc, Gles3MinorVersion::Automatic, Instance, InstanceDescriptor, InstanceFlags, PowerPreference, Queue, RenderPipelineDescriptor, RequestAdapterOptions, RequestDeviceError, Surface, SurfaceTarget, WindowHandle
};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Document, Element, Gpu, HtmlCanvasElement, HtmlElement, Navigator, Node, Window};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(start)]
pub async fn run() {
    let window: Window = if let Some(win) = create_window_handle() { win } else { panic!("FATAL: Window must exist for execution to continue") };
    let canvas: HtmlCanvasElement = get_canvas_context(&window);
    let instance = configure_webgpu_instance();
    let adapter = if let Some(adapter) = request_adapter(&instance, &window).await { adapter } else { panic!("WebGPUAdapter is needed to continue execution, unable to proceed")};
    let (device, queue) = if let Ok((d, q)) = request_device(&adapter).await {(d, q)}
                                         else { if let Ok((d, q)) = adapter.request_device(&DeviceDescriptor {
                                                                                                    label: None,
                                                                                                    required_features: wgpu::Features::all_webgpu_mask(),
                                                                                                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                                                                                                    memory_hints: wgpu::MemoryHints::Performance
                                                                                                }, None).await {(d, q)} else {panic!("")}};
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

fn configure_webgpu_instance() -> Instance {
    let out_dir = var("OUT_DIR").unwrap();
    Instance::new(InstanceDescriptor {
        backends: Backends::BROWSER_WEBGPU,
        flags: InstanceFlags::DEBUG,
        dx12_shader_compiler: Dxc { dxil_path: Some(PathBuf::from(&out_dir).join("dxc")), dxc_path: Some(PathBuf::from(&out_dir).join("dxc")) },
        gles_minor_version: Automatic
    })
}

async fn request_adapter(instance: &Instance, window: &Window) -> Option<Adapter>  {
    let surface = if let Ok(surface) = instance.create_surface(wgpu::SurfaceTarget::Canvas(get_canvas_context(&window))) { surface } else { panic!("Unable to create the rendering surface, unable to continue execution") };
    instance.request_adapter(&RequestAdapterOptions { compatible_surface: Some(&surface), force_fallback_adapter: false, power_preference: PowerPreference::HighPerformance}).await
}

async fn request_device(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError> {
    adapter.request_device(&DeviceDescriptor {
        required_features: wgpu::Features::all_webgpu_mask(),
        required_limits: wgpu::Limits::default(),
        label: None,
        memory_hints: wgpu::MemoryHints::Performance
    }, None).await
}