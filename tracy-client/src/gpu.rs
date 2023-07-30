use once_cell::sync::Lazy;
use std::{
    convert::TryInto,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc, Mutex,
    },
};

use crate::Client;

#[repr(u8)]
/// The API label associated with the given gpu context. The list here only includes
/// APIs that are currently supported by Tracy's own gpu implementations.
//
// Copied from `tracy-client-sys/tracy/common/TracyQueue.hpp:391`. Comment on enum states
// that the values are stable, due to potential serialization issues, so copying this enum
// shouldn't be a problem.
pub enum GpuContextType {
    /// Stand in for other types of contexts.
    Invalid = 0,
    /// An OpenGL context
    OpenGL = 1,
    /// A Vulkan context
    Vulkan = 2,
    /// An OpenCL context
    OpenCL = 3,
    /// A D3D12 context.
    Direct3D12 = 4,
    /// A D3D11 context.
    Direct3D11 = 5,
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
    /// Creates a new GPU context.
    ///
    /// - 'name' is the name of the context.
    /// - 'ty' is the type (backend) of the context.
    /// - 'gpu_timestamp' is the gpu side timestamp the corresponds (as close as possible) to this call.
    /// - 'period' is the period of the gpu clock in nanoseconds (setting 1.0 means the clock is 1GHz, 1000.0 means 1MHz, etc).
    pub fn new_gpu_context(
        self,
        name: Option<&str>,
        ty: GpuContextType,
        gpu_timestamp: i64,
        period: f32,
    ) -> Option<GpuContext> {
        #[cfg(feature = "enable")]
        {
            // We use a mutex to lock the context index to prevent races when using fetch_add.
            let mut context_index_guard = GPU_CONTEXT_INDEX.lock().unwrap();
            if *context_index_guard == 255 {
                return None;
            }
            let context = *context_index_guard;
            *context_index_guard += 1;
            drop(context_index_guard);

            unsafe {
                sys::___tracy_emit_gpu_new_context(sys::___tracy_gpu_new_context_data {
                    gpuTime: gpu_timestamp,
                    period,
                    context,
                    flags: 0,
                    type_: ty as u8,
                })
            };

            if let Some(name) = name {
                unsafe {
                    sys::___tracy_emit_gpu_context_name_serial(
                        sys::___tracy_gpu_context_name_data {
                            context,
                            name: name.as_ptr().cast(),
                            len: name.len().try_into().unwrap_or(u16::MAX),
                        },
                    )
                }
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
        {
            let srcloc = unsafe {
                sys::___tracy_alloc_srcloc_name(
                    line,
                    file.as_ptr().cast(),
                    file.len(),
                    function.as_ptr().cast(),
                    function.len(),
                    name.as_ptr().cast(),
                    name.len(),
                )
            };

            // Allocate two ids, one for the start value, one for the end.
            let query_id = self.span_index.fetch_add(2, Ordering::Relaxed);

            unsafe {
                sys::___tracy_emit_gpu_zone_begin_alloc_serial(sys::___tracy_gpu_zone_begin_data {
                    srcloc,
                    queryId: query_id,
                    context: self.context,
                })
            };

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
    pub fn end_zone(&mut self) -> Option<()> {
        #[cfg(feature = "enable")]
        {
            if self.ended {
                return None;
            }
            unsafe {
                sys::___tracy_emit_gpu_zone_end_serial(sys::___tracy_gpu_zone_end_data {
                    queryId: self.query_id.wrapping_add(1),
                    context: self.context,
                })
            };
            self.ended = true;
        }
        Some(())
    }

    ///
    pub fn upload_timestamp(self, start_timestamp: i64, end_timestamp: i64) {
        #[cfg(feature = "enable")]
        {
            if !self.ended {
                return;
            }
            unsafe {
                sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                    gpuTime: start_timestamp,
                    queryId: self.query_id,
                    context: self.context,
                })
            };

            unsafe {
                sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                    gpuTime: end_timestamp,
                    queryId: self.query_id.wrapping_add(1),
                    context: self.context,
                })
            };
        }
    }
}
