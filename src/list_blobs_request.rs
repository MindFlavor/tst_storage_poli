use crate::*;

#[derive(Debug, Clone)]
pub struct ListBlobsRequest<'a, T> {
    blob_client: &'a BlobClient<T>,
}

impl<'a, T> ListBlobsRequest<'a, T> {
    pub fn new(blob_client: &'a BlobClient<T>) -> Self {
        Self { blob_client }
    }

    pub fn execute(&self) {
        println!("\tlist blobs!");
    }
}
