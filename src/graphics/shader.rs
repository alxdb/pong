#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 4],
}

impl Vertex {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x4],
    };
}

impl From<[f64; 2]> for Vertex {
    fn from(value: [f64; 2]) -> Self {
        Vertex {
            position: [value[0] as f32, value[1] as f32, 0.0, 1.0],
        }
    }
}

impl From<&[f64; 2]> for Vertex {
    fn from(value: &[f64; 2]) -> Self {
        Vertex {
            position: [value[0] as f32, value[1] as f32, 0.0, 1.0],
        }
    }
}
