use once_cell::sync::Lazy;
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc, Mutex,
};

use crate::Client;

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
#[derive(Clone)]
pub struct GpuContext {
    #[cfg(feature = "enable")]
    client: Client,
    #[cfg(feature = "enable")]
    context: u8,
    #[cfg(feature = "enable")]
    span_index: Arc<AtomicU16>,
}
#[cfg(feature = "enable")]
static GPU_CONTEXT_INDEX: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(0));

///
pub struct GpuSpan {
    #[cfg(feature = "enable")]
    _client: Client,
    #[cfg(feature = "enable")]
    context: u8,
    #[cfg(feature = "enable")]
    // This is the index first query, the second query is always a wrapping_add(1) above this.
    query_id: u16,
    #[cfg(feature = "enable")]
    ended: bool,
}

impl Client {
    ///
    pub fn new_gpu_context(
        self,
        name: Option<&str>,
        ty: GpuContextType,
        gpu_timestamp: i64,
        period: f32,
    ) -> Option<GpuContext> {
        #[cfg(feature = "enable")]
        unsafe {
            // We use a mutex to lock the context index to prevent races when using fetch_add.
            let mut context_index_guard = GPU_CONTEXT_INDEX.lock().unwrap();
            if *context_index_guard == 255 {
                return None;
            }
            let context = *context_index_guard;
            *context_index_guard += 1;
            drop(context_index_guard);

            sys::___tracy_emit_gpu_new_context(std::mem::transmute(
                sys::___tracy_gpu_new_context_data {
                    gpuTime: gpu_timestamp,
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

            Some(GpuContext {
                client: self,
                context,
                span_index: Arc::new(AtomicU16::new(0)),
            })
        }
        #[cfg(not(feature = "enable"))]
        Some(GpuContext {})
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

            // Allocate two ids
            let query_id = self.span_index.fetch_add(2, Ordering::Relaxed);

            sys::___tracy_emit_gpu_zone_begin_alloc_serial(sys::___tracy_gpu_zone_begin_data {
                srcloc,
                queryId: query_id,
                context: self.context,
            });

            GpuSpan {
                _client: self.client.clone(),
                context: self.context,
                query_id,
                ended: false,
            }
        }
        #[cfg(not(feature = "enable"))]
        GpuSpan {}
    }
}

impl GpuSpan {
    ///
    pub fn end_zone(&mut self) -> bool {
        #[cfg(feature = "enable")]
        unsafe {
            if self.ended {
                return false;
            }
            sys::___tracy_emit_gpu_zone_end_serial(sys::___tracy_gpu_zone_end_data {
                queryId: self.query_id.wrapping_add(1),
                context: self.context,
            });
            self.ended = true;
            true
        }
    }

    ///
    pub fn upload_timestamp(self, start_timestamp: i64, end_timestamp: i64) {
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                gpuTime: start_timestamp,
                queryId: self.query_id,
                context: self.context,
            });

            sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                gpuTime: end_timestamp,
                queryId: self.query_id.wrapping_add(1),
                context: self.context,
            });
        }
    }
}
