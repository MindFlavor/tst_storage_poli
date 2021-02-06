mod blob_client;
mod container_client;
mod list_blobs_request;
mod list_containers_request;
mod put_blob_request;
pub use blob_client::BlobClient;
pub use container_client::{Container, ContainerClient};
pub use list_blobs_request::ListBlobsRequest;
pub use list_containers_request::ListContainersRequest;
pub use put_blob_request::{PutBlobRequestBlobStorage, PutBlobRequestStorageV2};
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Default)]
pub struct StorageV2 {}
#[derive(Debug, Clone, Copy, Default)]
pub struct BlobStorage {}

#[derive(Debug, Clone, Default)]
pub struct StorageClient<T> {
    t_t: PhantomData<T>,
}

impl<T> StorageClient<T> {
    pub fn into_container_client(self) -> ContainerClient<T> {
        ContainerClient {
            storage_client: self,
        }
    }
}

pub fn new_storage_v2() -> StorageClient<StorageV2> {
    StorageClient::<StorageV2>::default()
}

pub fn new_blob_storage() -> StorageClient<BlobStorage> {
    StorageClient::<BlobStorage>::default()
}

fn main() {
    // Storage V2
    let container = new_storage_v2().into_container_client();
    container.list_containers();

    let blob = container.into_blob_client();
    blob.put_blob().execute();
    blob.list_blobs().execute();
    blob.common_function();

    // BlobStorage
    let container = new_blob_storage().into_container_client();
    container.list_containers();

    let blob = container.into_blob_client();
    blob.put_blob().execute();
    blob.list_blobs().execute();
    blob.common_function();
}
