use std::cell::Cell;
use std::sync::Arc;

/// Manages a trace
pub trait Trace {
    /// Create a new span
    ///
    /// TODO: Should `name` be `&'static str`
    fn new_span(&self, name: &str) -> Option<SpanHandle>;

    /// Called when the span is closed
    fn close_span(&self, id: usize);
}

/// Handle to a a span.
pub struct SpanHandle {
    trace: Arc<Trace>,
    id: usize,
}

thread_local!(static CURRENT_TRACE: Cell<Option<*const Trace>> = Cell::new(None));

pub fn with_default<F, R>(trace: &Trace, f: F) -> R
where F: FnOnce() -> R,
{
    CURRENT_TRACE.with(|cell| {
        let prev = cell.get();

        // Ensure that the executor is removed from the thread-local context
        // when leaving the scope. This handles cases that involve panicking.
        struct Reset<'a> {
            cell: &'a Cell<Option<*const Trace>>,
            prev: Option<*const Trace>,
        }

        impl<'a> Drop for Reset<'a> {
            fn drop(&mut self) {
                self.cell.set(self.prev.take());
            }
        }

        unsafe fn hide_lt<'a>(p: *const (Trace + 'a)) -> *const (Trace + 'static) {
            use std::mem;
            mem::transmute(p)
        }

        let _reset = Reset {
            cell,
            prev,
        };

        // While scary, this is safe. The function takes a
        // `&Trace`, which guarantees that the reference lives for the
        // duration of `with_default`.
        //
        // Because we are always clearing the TLS value at the end of the
        // function, we can cast the reference to 'static which thread-local
        // cells require.
        let trace = unsafe { hide_lt(trace as &_ as *const _) };

        cell.set(Some(trace));

        f()
})
}

pub(crate) fn with_current<F, R>(f: F) -> R
where F: FnOnce(Option<&Trace>) -> R
{
    CURRENT_TRACE.with(|cell| {
        match cell.get() {
            Some(trace) => {
                // The lifetime is guaranteed as the reference is only passed
                // into a closure.
                let trace = unsafe { &*trace };
                f(Some(trace))
            }
            None => {
                f(None)
            }
        }
    })
}

impl SpanHandle {
    /// Create a new `SpanHandle`
    pub fn new(trace: Arc<Trace>, id: usize) -> SpanHandle {
        SpanHandle {
            trace,
            id,
        }
    }

    pub fn close(&self) {
        self.trace.close_span(self.id);
    }
}
