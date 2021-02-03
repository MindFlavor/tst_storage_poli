use crate::*;

#[derive(Debug, Clone, Default)]
pub struct ContainerClient<T> {
    pub(crate) storage_client: StorageClient<T>,
}

impl<TS> Container for ContainerClient<TS>
where
    TS: Debug,
{
    type StorageType = TS;

    fn list_containers(&self) -> ListContainersRequest<Self::StorageType> {
        println!("list_containers --> {:?}", self);
        ListContainersRequest::new(self)
    }
}

impl<T> ContainerClient<T> {
    pub fn into_blob_client(self) -> BlobClient<T> {
        BlobClient { blob_client: self }
    }
}
