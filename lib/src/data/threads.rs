use std::future::Future;
use tokio::task::JoinHandle;
use crate::utils::drop_static_mut_option;

static mut RUNTIME: Option<tokio::runtime::Runtime> = None;


pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where F: Future + Send + 'static,
          F::Output: Send + 'static,
{
    unsafe {
        RUNTIME.as_ref().unwrap().spawn(future)
    }
}

pub fn setup() {
    unsafe {
        RUNTIME = Some(tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_io()
            .enable_time()
            .build()
            .unwrap());
    }
}

pub fn teardown() {
    unsafe {
        drop_static_mut_option(&mut RUNTIME);
    }
}
