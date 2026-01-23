//! OpenTelemetry tracing integration.

use std::time::Instant;

/// Span context for distributed tracing
#[derive(Debug, Clone)]
pub struct SpanContext {
    /// Trace ID
    pub trace_id: String,
    /// Span ID
    pub span_id: String,
    /// Parent span ID (if any)
    pub parent_id: Option<String>,
}

impl SpanContext {
    /// Create a new root span context
    pub fn new_root() -> Self {
        Self {
            trace_id: generate_id(),
            span_id: generate_id(),
            parent_id: None,
        }
    }

    /// Create a child span context
    pub fn child(&self) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: generate_id(),
            parent_id: Some(self.span_id.clone()),
        }
    }
}

/// A span representing a unit of work
pub struct Span {
    /// Span name
    pub name: String,
    /// Span context
    pub context: SpanContext,
    /// Start time
    start: Instant,
    /// Span attributes
    attributes: Vec<(String, String)>,
}

impl Span {
    /// Create a new span
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            context: SpanContext::new_root(),
            start: Instant::now(),
            attributes: Vec::new(),
        }
    }

    /// Create a child span
    pub fn child(&self, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            context: self.context.child(),
            start: Instant::now(),
            attributes: Vec::new(),
        }
    }

    /// Add an attribute to the span
    pub fn set_attribute(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.push((key.into(), value.into()));
    }

    /// Get the elapsed time
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    /// End the span and log it
    pub fn end(self) {
        let duration = self.elapsed();
        tracing::info!(
            target: "tracing",
            trace_id = %self.context.trace_id,
            span_id = %self.context.span_id,
            parent_id = ?self.context.parent_id,
            name = %self.name,
            duration_ms = duration.as_millis() as u64,
            "span completed"
        );
    }
}

/// Execute a function within a span
pub fn span<F, T>(name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let span = Span::new(name);
    let result = f();
    span.end();
    result
}

/// Initialize tracing with OpenTelemetry
///
/// In production, configure with actual OTLP exporter:
/// ```rust,ignore
/// use opentelemetry::sdk::trace::TracerProvider;
/// use opentelemetry_otlp::WithExportConfig;
///
/// let tracer = opentelemetry_otlp::new_pipeline()
///     .tracing()
///     .with_exporter(opentelemetry_otlp::new_exporter().tonic())
///     .install_batch(opentelemetry::runtime::Tokio)?;
/// ```
pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement with opentelemetry crate
    // For now, this is a placeholder
    Ok(())
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    format!(
        "{:016x}{:016x}",
        duration.as_nanos() as u64,
        rand_u64()
    )
}

fn rand_u64() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    RandomState::new().build_hasher().finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_context_new() {
        let ctx = SpanContext::new_root();
        assert!(!ctx.trace_id.is_empty());
        assert!(!ctx.span_id.is_empty());
        assert!(ctx.parent_id.is_none());
    }

    #[test]
    fn test_span_context_child() {
        let parent = SpanContext::new_root();
        let child = parent.child();

        assert_eq!(child.trace_id, parent.trace_id);
        assert_ne!(child.span_id, parent.span_id);
        assert_eq!(child.parent_id, Some(parent.span_id));
    }

    #[test]
    fn test_span_function() {
        let result = span("test_operation", || {
            42
        });
        assert_eq!(result, 42);
    }
}
