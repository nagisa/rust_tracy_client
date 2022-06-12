//! Caches for buffers into which information is formatted.

use crossbeam_queue::ArrayQueue;
use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use tracing_subscriber::fmt::FormattedFields;

const DEFAULT_BUFFER_SIZE: usize = 64;

/// Provide a reference to a buffer for use-cases where it does not need to live outside of a
/// closure (such as would be the case for events).
pub(crate) fn with_event_buffer<F, C: FnOnce(&mut FormattedFields<F>)>(closure: C) {
    thread_local! {
        static BUFFER: std::cell::RefCell<Option<String>>  =
            RefCell::new(Some(String::with_capacity(DEFAULT_BUFFER_SIZE)));
    }
    BUFFER.with(|buffer| {
        let mut none = None;
        let mut storage = buffer.try_borrow_mut();
        let (buffer, storage) = match storage.as_mut().map(|b| (b.take(), b)) {
            Ok((Some(buffer), storage)) => (buffer, &mut **storage),
            _ => (String::with_capacity(DEFAULT_BUFFER_SIZE), &mut none),
        };
        let mut ff = FormattedFields::<F>::new(buffer);
        let panic = std::panic::catch_unwind(AssertUnwindSafe(|| closure(&mut ff)));
        if let Some(storage) = storage {
            ff.fields.clear();
            *storage = ff.fields;
        }
        if let Err(panic) = panic {
            std::panic::resume_unwind(panic);
        }
    });
}

#[derive(Clone)]
pub(crate) struct BufferPool<F> {
    pub(crate) pool: Arc<ArrayQueue<FormattedFields<F>>>,
    pub(crate) buffer_size: usize,
}

impl<F> BufferPool<F> {
    pub(crate) fn new(num_buffers: usize, buffer_size: usize) -> Self {
        let pool = Arc::new(ArrayQueue::new(num_buffers));
        for _ in 0..num_buffers {
            let _ = pool.push(FormattedFields::new(String::with_capacity(buffer_size)));
        }
        Self { pool, buffer_size }
    }

    pub(crate) fn remake<F2>(self, num_buffers: usize, buffer_size: usize) -> BufferPool<F2> {
        let pool = Arc::new(ArrayQueue::new(num_buffers));
        for _ in 0..num_buffers {
            let buffer = self.pool.pop().map(|v| FormattedFields::new(v.fields));
            let buffer =
                buffer.unwrap_or_else(|| FormattedFields::new(String::with_capacity(buffer_size)));
            let _ = pool.push(buffer);
        }
        BufferPool { pool, buffer_size }
    }

    pub(crate) fn get(&self) -> ReturnableBuffer<F> {
        let buffer = self.pool.pop();
        let buffer = buffer.map(|b| ReturnableBuffer {
            buffer: ManuallyDrop::new(b),
            pool: Some(Arc::clone(&self.pool)),
        });
        buffer.unwrap_or_else(|| ReturnableBuffer {
            buffer: ManuallyDrop::new(FormattedFields::new(String::with_capacity(
                DEFAULT_BUFFER_SIZE,
            ))),
            pool: None,
        })
    }
}

pub(crate) struct ReturnableBuffer<F> {
    buffer: ManuallyDrop<FormattedFields<F>>,
    // This is hella expensive :(
    pool: Option<Arc<ArrayQueue<FormattedFields<F>>>>,
}

impl<F> Drop for ReturnableBuffer<F> {
    fn drop(&mut self) {
        let mut buffer = unsafe {
            // SAFE: we ensure that `Drop` is the only place where this method is called on this
            // field, and `drop` is only ever called once by definition.
            ManuallyDrop::take(&mut self.buffer)
        };
        if let Some(pool) = &mut self.pool {
            buffer.fields.clear();
            let _ = pool.push(buffer);
        }
    }
}

impl<F> std::ops::Deref for ReturnableBuffer<F> {
    type Target = FormattedFields<F>;
    fn deref(&self) -> &Self::Target {
        &*self.buffer
    }
}

impl<F> std::ops::DerefMut for ReturnableBuffer<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.buffer
    }
}
