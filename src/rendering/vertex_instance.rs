use super::vertex_descriptor::VertexDesc;

#[repr(C)]
#[derive(Copy, Clone)] // might have to implement bytemuck attributes
pub struct VertexInstanceRaw {
    pub model: [[f32; 4]; 4],
    pub normal_matrix: [[f32; 4]; 4],
}

impl VertexDesc for VertexInstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VertexInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                // we are making our cube here shape for the noise.
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wpgu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 20]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },                
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 24]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },                
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 28]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
}

pub struct VertexInstance {
    pub position: cgmath::Vector3<f32>,
}

impl VertexInstance {
    pub fn to_raw(&self) -> VertexInstanceRaw {
        use cgmath::{Matrix, SquareMatrix};
        let model = cgmath::matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);

        let normal_matrix = model
        .invert()
        .expect("cant inverse this shit")
        .transpose();
        VertexInstanceRaw {
            model: model.into(),
            normal_matrix: normal_matrix.into(),
        }
    }

}
