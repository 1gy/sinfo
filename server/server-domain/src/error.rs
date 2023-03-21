#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error(transparent)]
    InfrastructureError(anyhow::Error),
}

pub type DomainResult<T> = Result<T, DomainError>;
