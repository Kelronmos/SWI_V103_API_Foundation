//! API routes and handlers

use serde::{Deserialize, Serialize};

/// API request/response envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Response data
    pub data: T,
    /// Response metadata
    pub meta: ApiMeta,
}

/// Metadata for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMeta {
    /// Request timestamp
    pub timestamp: String,
    /// Request ID for tracing
    pub request_id: String,
}
