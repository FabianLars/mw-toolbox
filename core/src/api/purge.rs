use crate::{error::ApiError, WikiClient};

pub async fn purge<C: AsRef<WikiClient>>(_client: C) -> Result<(), ApiError> {
    Err(ApiError::Unknown)
}
