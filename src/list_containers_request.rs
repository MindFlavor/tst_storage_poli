use crate::*;

#[derive(Debug, Clone)]
pub struct ListContainersRequest<'a, T> {
    container_client: &'a ContainerClient<T>,
}

impl<'a, T> ListContainersRequest<'a, T> {
    pub fn new(container_client: &'a ContainerClient<T>) -> Self {
        Self { container_client }
    }
}
