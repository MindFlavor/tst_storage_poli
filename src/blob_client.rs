use crate::*;

#[derive(Debug, Clone, Default)]
pub struct BlobClient<T> {
    pub(crate) blob_client: ContainerClient<T>,
}

impl BlobClient<BlobStorage> {
    pub fn put_blob(&self) -> PutBlobRequestBlobStorage {
        println!("put blob (BlobStorage) --> {:?}", self);
        PutBlobRequestBlobStorage::new(self)
    }
}

impl BlobClient<StorageV2> {
    pub fn put_blob(&self) -> PutBlobRequestStorageV2 {
        println!("put blob (V2) --> {:?}", self);
        PutBlobRequestStorageV2::new(self)
    }
}

impl<T> BlobClient<T>
where
    T: std::fmt::Debug,
{
    pub fn common_function(&self) {
        println!("common_function (common) --> {:?}", self);
    }

    pub fn list_blobs(&self) -> ListBlobsRequest<'_, T> {
        println!("list blobs (common) --> {:?}", self);
        ListBlobsRequest::new(self)
    }
}
