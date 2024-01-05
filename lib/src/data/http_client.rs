use once_cell::sync::OnceCell;
use reqwest::blocking::Client as BlockingHttpClient;
use reqwest::Client as AsyncHttpClient;
use rustls::{ClientConfig, RootCertStore};

static BLOCKING_CLIENT: OnceCell<Option<BlockingHttpClient>> = OnceCell::new();
static ASYNC_CLIENT: OnceCell<Option<AsyncHttpClient>> = OnceCell::new();

pub fn get_http_client() -> Option<&'static BlockingHttpClient> {
    BLOCKING_CLIENT.get_or_init(create_blocking_client).as_ref()
}

pub fn get_async_client() -> Option<&'static AsyncHttpClient> {
    ASYNC_CLIENT.get_or_init(create_async_client).as_ref()
}


fn create_blocking_client() -> Option<BlockingHttpClient> {
    let roots: RootCertStore = match get_cert_store() {
        Some(value) => value,
        None => {
            return None;
        }
    };

    let builder = get_client_config(roots);


    Some(BlockingHttpClient::builder()
        .use_preconfigured_tls(builder)
        .build()
        .unwrap())
}

fn create_async_client() -> Option<AsyncHttpClient> {
    let roots: RootCertStore = match get_cert_store() {
        Some(value) => value,
        None => {
            return None;
        }
    };

    let builder = get_client_config(roots);


    Some(AsyncHttpClient::builder()
        .use_preconfigured_tls(builder)
        .build()
        .unwrap())
}

fn get_client_config(roots: RootCertStore) -> ClientConfig {
    ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth()
}

fn get_cert_store() -> Option<RootCertStore> {
    let mut roots = RootCertStore::empty();
    let result = rustls_native_certs::load_native_certs();
    match result {
        Ok(certs) => {
            for cert in certs {
                match roots.add(&rustls::Certificate(cert.0)) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Could not load platform certs!");
            return None;
        }
    }
    Some(roots)
}