use futures::executor::block_on;
use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracy_client::frame_image;

pub fn main() {
    tracy_client::Client::start();
    let instance = wgpu::Instance::new(Default::default());
    let adapter = block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: Default::default(),
            required_limits: Default::default(),
            memory_hints: Default::default(),
        },
        None,
    ))
    .unwrap();

    // We're just going to render to a decently large texture, pretending that that's a window.
    // For this to work in a real application, you need to render to a texture which is then
    // drawn back to the window surface
    let mut game = Game::new(&device, 1280, 720);
    for _ in 1..200 {
        game.render(&device, &queue);
        sleep(Duration::from_millis(20));
    }
}

enum CaptureBufferStatus {
    /// This buffer is free to use
    Free,

    /// This buffer has been reserved and commands to capture a texture have been encoded
    Capturing { frame_num: u64 },

    /// The capture commands have been submitted and we're waiting to map the buffer (when the GPU is ready)
    Mapping {
        frame_num: u64,
        ready: Arc<AtomicBool>,
    },
}

struct CaptureBuffer {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    buffer: wgpu::Buffer,
    status: CaptureBufferStatus,
}

impl CaptureBuffer {
    /// Width of a capture texture
    /// This MUST be a multiple of 4.
    const WIDTH: u32 = 320;

    /// Height of a capture texture
    /// This MUST be a multiple of 4.
    const HEIGHT: u32 = 180;

    /// Amount of capture buffers to use
    /// If there's more than this much in flight, no more will be captured and we may lose frames in Tracy.
    /// If there's too many, we're wasting memory.
    /// Tune it to your preferences and use case, or make it dynamic!
    const AMOUNT: usize = 3;

    fn new(device: &wgpu::Device) -> Self {
        // This will stretch the captured image if the aspect ratio doesn't match
        // In a real application you'll want to resize these buffers to preserve the aspect ratio
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: Self::WIDTH,
                height: Self::HEIGHT,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // 4 bytes per pixel (Tracy wants RGBA)
        let bytes_per_row = Self::WIDTH * 4;
        let padded_bytes_per_row =
            wgpu::util::align_to(bytes_per_row, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (padded_bytes_per_row * Self::HEIGHT * 4) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Self {
            texture,
            view,
            buffer,
            status: CaptureBufferStatus::Free,
        }
    }

    pub fn submit_to_tracy(&self, current_frame: u64, frame_num: u64) {
        // If we're so far behind we couldn't even fit into a u8, just drop it.
        let Some(offset) = current_frame
            .checked_sub(frame_num)
            .and_then(|o| u8::try_from(o).ok())
        else {
            return;
        };

        // Tracy needs a raw rgba image with no padding, so we have to remove it
        let mut unpadded = Vec::with_capacity(Self::WIDTH as usize * Self::HEIGHT as usize * 4);
        let padded_width =
            wgpu::util::align_to(Self::WIDTH * 4, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        let buffer = self.buffer.slice(..).get_mapped_range();
        for row in 0..Self::HEIGHT {
            let start = row as usize * padded_width as usize;
            let end = start + (Self::WIDTH as usize * 4);
            unpadded.extend_from_slice(&buffer[start..end]);
        }

        // Submit it!
        frame_image(
            &unpadded,
            Self::WIDTH as u16,
            Self::HEIGHT as u16,
            offset,
            false,
        );
    }
}

/// An amazing game that renders a spinning triangle. Technology!
struct Game {
    renderer: TriangleRenderer,
    frame_buffer_view: wgpu::TextureView,
    blit_pipeline: wgpu::RenderPipeline,
    blit_frame_buffer_bind_group: wgpu::BindGroup,
    frame_count: u64,
    frame_captures: [CaptureBuffer; CaptureBuffer::AMOUNT],
}

impl Game {
    fn new(device: &wgpu::Device, width: u32, height: u32) -> Game {
        let renderer = TriangleRenderer::new(device);

        // A trivial blit (copy) pipeline; used both to render the game, and to take screenshots for Tracy
        let blit_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("blit.wgsl"))),
        });
        let blit_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });
        let blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&blit_bind_group_layout],
            push_constant_ranges: &[],
        });
        let blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&blit_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &blit_shader_module,
                entry_point: None,
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &blit_shader_module,
                entry_point: None,
                compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
            }),
            multiview: None,
            cache: None,
        });

        let blit_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // We'll render the game to an intermediary texture (frame_buffer),
        // this allows us to take copies of the texture at will (aka screenshots)
        let frame_buffer = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let frame_buffer_view = frame_buffer.create_view(&Default::default());
        let blit_frame_buffer_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &blit_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&blit_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&frame_buffer_view),
                },
            ],
        });
        let tracy_frame_buffers: [CaptureBuffer; CaptureBuffer::AMOUNT] =
            core::array::from_fn(|_| CaptureBuffer::new(device));

        Self {
            renderer,
            frame_buffer_view,
            blit_pipeline,
            blit_frame_buffer_bind_group,
            frame_count: 0,
            frame_captures: tracy_frame_buffers,
        }
    }

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder = device.create_command_encoder(&Default::default());
        self.renderer
            .render(queue, &mut encoder, &self.frame_buffer_view);

        // Here's where we'd blit it to the surface... if we had one...

        // Before we submit everything, but _after the frame buffer is finished_, let's try to capture an image
        if let Some(capture) = self
            .frame_captures
            .iter_mut()
            .find(|c| matches!(c.status, CaptureBufferStatus::Free))
        {
            // We've got a free capture slot! First blit our frame buffer over to the texture
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &capture.view,
                    resolve_target: None,
                    ops: Default::default(),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.blit_pipeline);
            render_pass.set_bind_group(0, &self.blit_frame_buffer_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
            drop(render_pass);

            // Then copy the texture to a buffer
            encoder.copy_texture_to_buffer(
                wgpu::ImageCopyTexture {
                    texture: &capture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyBuffer {
                    buffer: &capture.buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(wgpu::util::align_to(
                            CaptureBuffer::WIDTH * 4,
                            wgpu::COPY_BYTES_PER_ROW_ALIGNMENT,
                        )),
                        rows_per_image: Some(CaptureBuffer::HEIGHT),
                    },
                },
                capture.texture.size(),
            );

            // And mark it ready for mapping (that has to happen AFTER this encoder is submitted)
            capture.status = CaptureBufferStatus::Capturing {
                frame_num: self.frame_count,
            };
        }

        queue.submit(Some(encoder.finish()));

        // Here's where we'd present the surface

        // Tell tracy that's the end of a frame - it's usually expected that this is immediately after presenting to the surface
        tracy_client::frame_mark();
        // We're tracking "number of frames we've told Tracy about" - so increment it here vs anywhere else
        self.frame_count += 1;

        // Now that we've submitted everything, let's find out if any of our captures are ready
        for capture in &mut self.frame_captures {
            match &capture.status {
                // Nothing to do
                CaptureBufferStatus::Free => continue,

                // We'll need to try and map this
                CaptureBufferStatus::Capturing { frame_num } => {
                    let ready = Arc::new(AtomicBool::new(false));
                    capture.status = CaptureBufferStatus::Mapping {
                        frame_num: *frame_num,
                        ready: ready.clone(),
                    };
                    capture
                        .buffer
                        .slice(..)
                        .map_async(wgpu::MapMode::Read, move |_| {
                            ready.store(true, Ordering::Relaxed);
                        });
                }

                // If this is ready, submit it to Tracy and then free it up for a new capture
                CaptureBufferStatus::Mapping { frame_num, ready } => {
                    if ready.load(Ordering::Relaxed) {
                        capture.submit_to_tracy(self.frame_count, *frame_num);
                        capture.buffer.unmap();
                        capture.status = CaptureBufferStatus::Free;
                    }
                }
            }
        }
    }
}

struct TriangleRenderer {
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    start: Instant,
}

impl TriangleRenderer {
    fn new(device: &wgpu::Device) -> Self {
        let triangle_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("triangle.wgsl"))),
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &triangle_shader_module,
                entry_point: None,
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &triangle_shader_module,
                entry_point: None,
                compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
            }),
            multiview: None,
            cache: None,
        });
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<f32>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Self {
            pipeline,
            buffer,
            bind_group,
            start: Instant::now(),
        }
    }

    fn render(
        &mut self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
    ) {
        queue.write_buffer(
            &self.buffer,
            0,
            &self.start.elapsed().as_secs_f32().to_ne_bytes(),
        );
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
