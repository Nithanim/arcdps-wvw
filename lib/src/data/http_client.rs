use once_cell::sync::OnceCell;
use reqwest::blocking::Client;
use rustls::{ClientConfig, RootCertStore};

static HTTP_CLIENT: OnceCell<Option<Client>> = OnceCell::new();

pub fn get_http_client() -> Option<&'static Client> {
    HTTP_CLIENT.get_or_init(create_http_client).as_ref()
}

fn create_http_client() -> Option<Client> {
    let roots: RootCertStore = match get_cert_store() {
        Some(value) => value,
        None => {
            return None;
        }
    };

    let builder = get_client_config(roots);


    Some(Client::builder()
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