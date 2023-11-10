use crate::{CacheControl, QueryEnv, Request, ServerError};

/// Represents a GraphQL validator
#[async_trait::async_trait]
pub trait Validator: Unpin + Clone + Send + Sync + 'static {
    /// Validate a GraphQL query
    async fn validate(
        &self,
        request: Request,
    ) -> Result<(QueryEnv, CacheControl), Vec<ServerError>>;
}
