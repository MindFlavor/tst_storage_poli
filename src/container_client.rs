use crate::*;

#[derive(Debug, Clone, Default)]
pub struct ContainerClient<T> {
    pub(crate) storage_client: StorageClient<T>,
}

pub trait Container {
    fn list_containers(&self) -> ListContainersRequest<'_, Self>;
}

impl<T> ContainerClient<T>
where
    T: std::fmt::Debug,
{
    pub fn into_blob_client(self) -> BlobClient<T> {
        BlobClient { blob_client: self }
    }
}

impl<T> Container for ContainerClient<T>
where
    T: std::fmt::Debug,
{
    fn list_containers(&self) -> ListContainersRequest<'_, Self> {
        println!("list_containers (common) --> {:?}", self);
        ListContainersRequest::new(self)
    }
}
