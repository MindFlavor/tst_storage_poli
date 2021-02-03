use crate::*;

#[derive(Debug, Clone, Default)]
pub struct BlobClient<T> {
    pub(crate) blob_client: ContainerClient<T>,
}

impl BlobClient<BlobStorage> {
    pub fn put_blob(&self) -> PutBlobRequestBlobStorage {
        println!("list_containers --> {:?}", self);
        PutBlobRequestBlobStorage::new(self)
    }
}

impl BlobClient<StorageV2> {
    pub fn put_blob(&self) -> PutBlobRequestStorageV2 {
        println!("list_containers --> {:?}", self);
        PutBlobRequestStorageV2::new(self)
    }
}
