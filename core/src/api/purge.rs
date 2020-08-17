use crate::WikiClient;

pub async fn purge<C: AsRef<WikiClient>>(_client: C) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("NOT IMPLEMENTED"))
}
