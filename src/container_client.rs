use crate::*;

#[derive(Debug, Clone, Default)]
pub struct ContainerClient<T> {
    pub(crate) storage_client: StorageClient<T>,
}

impl<T> ContainerClient<T>
where
    T: std::fmt::Debug,
{
    pub fn into_blob_client(self) -> BlobClient<T> {
        BlobClient { blob_client: self }
    }

    pub fn list_containers(&self) -> ListContainersRequest<T> {
        println!("list_containers --> {:?}", self);
        ListContainersRequest::new(self)
    }
}
