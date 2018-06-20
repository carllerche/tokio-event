//! Tokio event system
//!
//! Each span encapsulates the following state:
//!
//! * An operation name
//! * A start timestamp
//! * A finish timestamp
//! * A set of zero or more key:value Span Tags. The keys must be strings. The
//!   values may be strings, bools, or numeric types.
//! * A SpanContext (see below)
//! * References to zero or more causally-related Spans (via the SpanContext of
//!   those related Spans)

pub mod collect;

/// Event handle
pub struct Span {
    id: SpanId,
}

/// A span identifier
pub struct SpanId {
    handle: Option<collect::SpanHandle>,
}

pub fn span(name: &str) -> Span {
    let handle = collect::with_current(|trace| {
        // TODO: What should happen?
        trace.unwrap().new_span(name)
    });

    // Create the span identifier
    let id = SpanId { handle };

    // Create the handle
    Span { id }
}

impl Span {
    pub fn follows_from(&mut self, identifier: &SpanId) {
        unimplemented!();
    }

    pub fn tag<T>(&mut self, key: &str, value: T) {
        unimplemented!();
    }

    pub fn log(&mut self) {
        unimplemented!();
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        if let Some(ref handle) = self.id.handle {
            handle.close();
        }
    }
}
