use crate::*;

#[derive(Debug, Clone)]
pub struct ListContainersRequest<'a, C: Container + ?Sized> {
    container_client: &'a C,
}

impl<'a, C: Container + ?Sized> ListContainersRequest<'a, C> {
    pub fn new(container_client: &'a C) -> Self {
        Self { container_client }
    }
}
