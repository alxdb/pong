use nalgebra as na;
use std::{ops::Range, rc::Rc};
use wgpu::util::DeviceExt;

use winit::window::Window;

mod shader;

use self::shader::Vertex;

pub struct Object {
    vertex_buffer: wgpu::Buffer,
    vertices: Range<u32>,
}

pub struct ObjectDescriptor {
    vertices: Vec<Vertex>,
}

struct Uniforms {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

pub struct Graphics<'a> {
    window: &'a Window,
    adapter: wgpu::Adapter,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    uniforms: Uniforms,
}

impl<'a> Graphics<'a> {
    pub fn new(window: &Window) -> Graphics {
        use crate::utils::BlockFuture;

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .block()
            .expect("Could not find suitable adapter");
        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .block()
            .unwrap();

        let surface_config = Self::configure_surface(&surface, &adapter, &device, window);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let projection = Self::create_uniforms(&surface_config);
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[projection]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&uniform_bind_group_layout],
            ..Default::default()
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::LAYOUT],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(surface_config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Graphics {
            window,
            adapter,
            surface,
            device,
            queue,
            pipeline,
            uniforms: Uniforms {
                buffer: uniform_buffer,
                bind_group: uniform_bind_group,
            },
        }
    }

    fn configure_surface(
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        window: &Window,
    ) -> wgpu::SurfaceConfiguration {
        let size = window.inner_size();
        let config = surface
            .get_default_config(adapter, size.width, size.height)
            .expect("Surface not supported by adapter");
        surface.configure(device, &config);
        config
    }

    fn projection(surface_config: &wgpu::SurfaceConfiguration) -> [[f32; 4]; 4] {
        let scale = 1000.;
        let x = (surface_config.width as f64 / 2.) / scale;
        let y = (surface_config.height as f64 / 2.) / scale;

        na::Orthographic3::new(-x, x, -y, y, 1., -1.)
            .as_matrix()
            .cast::<f32>()
            .into()
    }

    fn create_uniforms(surface_config: &wgpu::SurfaceConfiguration) -> shader::Uniforms {
        shader::Uniforms {
            proj: Self::projection(surface_config),
        }
    }

    pub fn on_resize(&self) {
        let config =
            Self::configure_surface(&self.surface, &self.adapter, &self.device, self.window);
        let projection = Self::create_uniforms(&config);
        self.queue.write_buffer(
            &self.uniforms.buffer,
            0,
            bytemuck::cast_slice(&[projection]),
        );
    }

    pub fn draw<'b>(&self, objects: impl IntoIterator<Item = &'b Object>) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.uniforms.bind_group, &[]);
            for object in objects.into_iter() {
                pass.set_vertex_buffer(0, object.vertex_buffer.slice(..));
                pass.draw(object.vertices.clone(), 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

impl Object {
    pub fn new(graphics: &Graphics, descriptor: ObjectDescriptor) -> Self {
        Object {
            vertex_buffer: graphics
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&descriptor.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            vertices: (0..descriptor.vertices.len() as u32),
        }
    }
}

impl ObjectDescriptor {
    pub fn circle(n_segments: u32, radius: f64) -> Self {
        use std::f64::consts::TAU;

        let origin = [0.0, 0.0];
        let vertices = (0..=n_segments)
            .map(|n| (TAU / n_segments as f64) * n as f64)
            .map(|angle| [radius * angle.cos(), radius * angle.sin()])
            .intersperse(origin)
            .map(Into::into)
            .collect();

        Self { vertices }
    }

    pub fn rect(width: f64, height: f64) -> Self {
        macro_rules! cartesian_product {
            ($xs:expr, $ys:expr) => {
                $xs.map(|x| $ys.map(|y| [x, y])).flatten().iter()
            };
        }

        let vertices = cartesian_product!([-width, width], [-height, height])
            .map(Into::into)
            .collect();

        Self { vertices }
    }
}
