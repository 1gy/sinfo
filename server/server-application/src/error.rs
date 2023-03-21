use server_domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    DomainError(#[from] DomainError),
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
