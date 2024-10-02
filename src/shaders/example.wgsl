@vertex
fn vs_main(@location(0) position: vec4<f32>, @location(1) color: vec4<f32>) -> @builtin(position) vec4<f32> {
    return position;
}

@fragment
fn fs_main(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}