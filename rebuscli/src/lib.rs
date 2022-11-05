mod client;

pub use client::RebusdClient;
use futures::lock::Mutex;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;

pub type Result<T> = anyhow::Result<T>;

static GLOBAL_CLIENT_POOL: OnceCell<HashMap<String, Arc<Mutex<RebusdClient>>>> = OnceCell::new();

pub fn get_client(endpoint: &str) -> &'static Mutex<RebusdClient> {
    GLOBAL_CLIENT_POOL
        .get()
        .expect("global rebus GRPC client pool must be initialized...")
        .get(endpoint)
        .expect("rebus GRPC client with specific endpoint must be initialized...")
}

pub fn set_client_pool(client_pool: HashMap<String, Arc<Mutex<RebusdClient>>>) {
    if GLOBAL_CLIENT_POOL.set(client_pool).is_err() {
        eprintln!("Global rebus GRPC client pool has already been set");
    }
}
