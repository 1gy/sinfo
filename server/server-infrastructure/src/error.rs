use server_domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum InfrastructureError {
    #[error("unsupported cpu")]
    UnsupportedCpu(),
}

impl Into<DomainError> for InfrastructureError {
    fn into(self) -> DomainError {
        DomainError::InfrastructureError(self.into())
    }
}

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;
