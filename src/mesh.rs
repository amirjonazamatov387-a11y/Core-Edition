use wgpu::util::DeviceExt;

pub const CHUNK_SIZE: i32 = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x3];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

struct Face {
    normal: [f32; 3],
    neighbor: [i32; 3],
    corners: [[f32; 3]; 4],
}

fn voxel_faces() -> [Face; 6] {
    [
        Face {
            normal: [0.0, 0.0, 1.0],
            neighbor: [0, 0, 1],
            corners: [
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
            ],
        },
        Face {
            normal: [0.0, 0.0, -1.0],
            neighbor: [0, 0, -1],
            corners: [
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
            ],
        },
        Face {
            normal: [1.0, 0.0, 0.0],
            neighbor: [1, 0, 0],
            corners: [
                [0.5, -0.5, 0.5],
                [0.5, -0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, 0.5, 0.5],
            ],
        },
        Face {
            normal: [-1.0, 0.0, 0.0],
            neighbor: [-1, 0, 0],
            corners: [
                [-0.5, -0.5, -0.5],
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [-0.5, 0.5, -0.5],
            ],
        },
        Face {
            normal: [0.0, 1.0, 0.0],
            neighbor: [0, 1, 0],
            corners: [
                [-0.5, 0.5, 0.5],
                [0.5, 0.5, 0.5],
                [0.5, 0.5, -0.5],
                [-0.5, 0.5, -0.5],
            ],
        },
        Face {
            normal: [0.0, -1.0, 0.0],
            neighbor: [0, -1, 0],
            corners: [
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
            ],
        },
    ]
}

fn occupied(x: i32, y: i32, z: i32) -> bool {
    x >= 0 && x < CHUNK_SIZE && y >= 0 && y < CHUNK_SIZE && z >= 0 && z < CHUNK_SIZE
}

fn block_color(y: i32) -> [f32; 3] {
    if y >= CHUNK_SIZE - 1 {
        [0.33, 0.72, 0.28]
    } else if y < CHUNK_SIZE / 4 {
        [0.48, 0.50, 0.54]
    } else {
        [0.62, 0.45, 0.28]
    }
}

pub fn build_chunk_mesh() -> (Vec<Vertex>, Vec<u32>) {
    let faces = voxel_faces();
    let exterior_faces = (CHUNK_SIZE * CHUNK_SIZE * 6) as usize;
    let mut vertices = Vec::with_capacity(exterior_faces * 4);
    let mut indices = Vec::with_capacity(exterior_faces * 6);
    let origin = (CHUNK_SIZE as f32 - 1.0) * 0.5;

    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let color = block_color(y);
            for x in 0..CHUNK_SIZE {
                let center = [
                    x as f32 - origin,
                    y as f32 - origin,
                    z as f32 - origin,
                ];

                for face in &faces {
                    let nx = x + face.neighbor[0];
                    let ny = y + face.neighbor[1];
                    let nz = z + face.neighbor[2];
                    if occupied(nx, ny, nz) {
                        continue;
                    }

                    let start = vertices.len() as u32;
                    for corner in face.corners {
                        vertices.push(Vertex {
                            position: [
                                center[0] + corner[0],
                                center[1] + corner[1],
                                center[2] + corner[2],
                            ],
                            color,
                            normal: face.normal,
                        });
                    }
                    indices.extend_from_slice(&[
                        start,
                        start + 1,
                        start + 2,
                        start,
                        start + 2,
                        start + 3,
                    ]);
                }
            }
        }
    }

    (vertices, indices)
}

pub fn create_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let (vertices, indices) = build_chunk_mesh();
    let num_indices = indices.len() as u32;

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Voxel Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Voxel Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    (vertex_buffer, index_buffer, num_indices)
}
