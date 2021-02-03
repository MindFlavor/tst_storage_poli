mod blob_client;
mod container_client;
mod list_containers_request;
mod put_blob_request;
pub use blob_client::BlobClient;
pub use container_client::ContainerClient;
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

pub trait Container {
    type StorageType;

    fn list_containers(&self) -> ListContainersRequest<Self::StorageType>;
}

//pub trait Blob {
//    type StorageType;
//
//    fn put_blob(&self) -> PutBlobRequest<Self::StorageType>;
//}

fn main() {
    let cat = new_storage_v2().into_container_client();
    let mouse = new_blob_storage().into_container_client();

    cat.list_containers();
    mouse.list_containers();

    let cat = cat.into_blob_client();
    let mouse = mouse.into_blob_client();

    let request_cat = cat.put_blob();
    let request_mouse = mouse.put_blob();

    request_cat.execute();
    request_mouse.execute();
}
