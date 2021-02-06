use crate::*;

#[derive(Debug, Clone)]
pub struct PutBlobRequestStorageV2<'a> {
    blob_client: &'a BlobClient<StorageV2>,
}

impl<'a> PutBlobRequestStorageV2<'a> {
    pub fn new(blob_client: &'a BlobClient<StorageV2>) -> Self {
        Self { blob_client }
    }

    pub fn execute(&self) {
        println!("\texecute storage V2!");
    }
}

#[derive(Debug, Clone)]
pub struct PutBlobRequestBlobStorage<'a> {
    blob_client: &'a BlobClient<BlobStorage>,
}

impl<'a> PutBlobRequestBlobStorage<'a> {
    pub fn new(blob_client: &'a BlobClient<BlobStorage>) -> Self {
        Self { blob_client }
    }

    pub fn execute(&self) {
        println!("\texecute blob storage!");
    }
}
