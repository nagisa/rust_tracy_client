use std::sync::atomic::{AtomicU16, AtomicU8, Ordering};

use crate::Client;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct ___tracy_gpu_new_context_data {
    gpu_time: i64,
    period: f32,
    context: u8,
    flags: u8,
    type_: u8,
}

#[repr(u8)]
///
pub enum GpuContextType {
    ///
    Invalid,
    ///
    OpenGL,
    ///
    Vulkan,
    ///
    OpenCL,
    ///
    Direct3D12,
    ///
    Direct3D11,
}

///
pub struct GpuContext {
    #[cfg(feature = "enable")]
    _client: Client,
    #[cfg(feature = "enable")]
    context: u8,
}
#[cfg(feature = "enable")]
static GPU_CONTEXT_INDEX: AtomicU8 = AtomicU8::new(0);

///
pub struct GpuSpan {
    #[cfg(feature = "enable")]
    _client: Client,
    #[cfg(feature = "enable")]
    context: u8,
    #[cfg(feature = "enable")]
    start_query_id: u16,
    #[cfg(feature = "enable")]
    end_query_id: Option<u16>,
}
#[cfg(feature = "enable")]
static GPU_SPAN_INDEX: AtomicU16 = AtomicU16::new(0);

impl Client {
    ///
    pub fn new_gpu_context(
        self,
        name: Option<&str>,
        ty: GpuContextType,
        gpu_timestamp: i64,
        period: f32,
    ) -> GpuContext {
        #[cfg(feature = "enable")]
        unsafe {
            let context = GPU_CONTEXT_INDEX.fetch_add(1, Ordering::Relaxed);

            sys::___tracy_emit_gpu_new_context(std::mem::transmute(
                ___tracy_gpu_new_context_data {
                    gpu_time: gpu_timestamp,
                    period,
                    context,
                    flags: 0,
                    type_: ty as u8,
                },
            ));

            if let Some(name) = name {
                sys::___tracy_emit_gpu_context_name_serial(sys::___tracy_gpu_context_name_data {
                    context,
                    name: name.as_ptr() as *const i8,
                    len: name.len().min(u16::MAX as usize) as u16,
                })
            }

            GpuContext {
                _client: self,
                context,
            }
        }
        #[cfg(not(feature = "enable"))]
        GpuContext {}
    }
}

impl GpuContext {
    ///
    pub fn span_alloc(&self, name: &str, function: &str, file: &str, line: u32) -> GpuSpan {
        #[cfg(feature = "enable")]
        unsafe {
            let srcloc = sys::___tracy_alloc_srcloc_name(
                line,
                file.as_ptr().cast(),
                file.len(),
                function.as_ptr().cast(),
                function.len(),
                name.as_ptr().cast(),
                name.len(),
            );

            let query_id = GPU_SPAN_INDEX.fetch_add(1, Ordering::Relaxed);

            sys::___tracy_emit_gpu_zone_begin_alloc_serial(sys::___tracy_gpu_zone_begin_data {
                srcloc,
                queryId: query_id,
                context: self.context,
            });

            GpuSpan {
                _client: Client::running().unwrap(),
                context: self.context,
                start_query_id: query_id,
                end_query_id: None,
            }
        }
        #[cfg(not(feature = "enable"))]
        GpuSpan {}
    }
}

impl GpuSpan {
    ///
    pub fn end_zone(&mut self) {
        #[cfg(feature = "enable")]
        unsafe {
            let query_id = GPU_SPAN_INDEX.fetch_add(1, Ordering::Relaxed);
            sys::___tracy_emit_gpu_zone_end_serial(sys::___tracy_gpu_zone_end_data {
                queryId: query_id,
                context: self.context,
            });
            self.end_query_id = Some(query_id);
        }
    }

    ///
    pub fn upload_timestamp(self, start_timestamp: i64, end_timestamp: i64) {
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                gpuTime: start_timestamp,
                queryId: self.start_query_id,
                context: self.context,
            });

            sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                gpuTime: end_timestamp,
                queryId: self.end_query_id.expect("called upload_timestamp without calling end_zone first"),
                context: self.context,
            });
        }
    }
}
