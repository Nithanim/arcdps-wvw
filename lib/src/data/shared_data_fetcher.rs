use std::sync::{mpsc};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Instant};
use crate::data::{SharedData, threads};
use crate::data::shared_data_fetcher_thread::download_and_get_data_async;
use crate::settings::{get_settings, Settings};
use crate::utils::{drop_static_mut_option, swap_static_mut_option};

static mut RUNTIME: Option<tokio::runtime::Runtime> = None;
static SHUTDOWN: AtomicBool = AtomicBool::new(false);

static mut RECEIVER: Option<Receiver<Option<SharedData>>> = None;

static mut SHARED_DATA_FETCHER: Option<SharedDataFetcher> = None;

pub fn get_shared_data() -> Option<&'static SharedData> {
    unsafe {
        SHARED_DATA_FETCHER.as_ref().and_then(|c| c.shared_data.as_ref())
    }
}

struct SharedDataFetcher {
    receiver: Receiver<Option<SharedData>>,
    sender: Sender<Option<SharedData>>,
    last_fetch: Option<Instant>,
    currently_fetching: bool,
    shared_data: Option<SharedData>,
}

impl SharedDataFetcher {
    fn tick(&'static mut self) {
        match self.receiver.try_recv() {
            Ok(data) => {
                self.shared_data = data;
                self.currently_fetching = false;
                self.last_fetch = Some(Instant::now());
            }
            Err(_) => {}
        }

        if settings_need_data(get_settings()) {
            if !self.currently_fetching {
                match self.last_fetch {
                    None => self.do_fetch(),
                    Some(last_fetch) => {
                        let millis_elapsed = last_fetch.elapsed().as_millis() as u64;
                        if millis_elapsed > 2800 {
                            self.do_fetch();
                        }
                    }
                }
            }
        }
    }

    fn do_fetch(&'static mut self) {
        self.currently_fetching = true;
        threads::spawn(self.do_fetch_());
    }

    async fn do_fetch_(&mut self) {
        let data = download_and_get_data_async().await;
        self.sender.send(data);
    }
}


pub fn tick() {
    unsafe {
        match &mut SHARED_DATA_FETCHER {
            None => {}
            Some(shared_data_fetcher) => {
                shared_data_fetcher.tick();
            }
        }
    }
}

pub fn setup() {
    let (channel_sender, channel_receiver) = mpsc::channel::<Option<SharedData>>();
    unsafe {
        SHARED_DATA_FETCHER = Some(SharedDataFetcher {
            sender: channel_sender,
            receiver: channel_receiver,
            last_fetch: None,
            currently_fetching: false,
            shared_data: None,
        });
    }
}

pub fn shutdown() {
    unsafe {
        // TODO All this nonsense around static mut vars is inherently not correctly managed and should be replaced
        // But for GW2 this has to suffice for now.

        // THIS IS CURRENTLY NOT SAFE SINCE THE ASYNC MIGHT BE DOING STUFF AND MIGHT REFERENCE SOMETHING, MAYBE
        drop_static_mut_option(&mut SHARED_DATA_FETCHER);
    }
}

fn settings_need_data(settings: &Settings) -> bool {
    settings.show_red || settings.show_green || settings.show_blue || settings.show_eternal || settings.show_objectives_overlay
}

