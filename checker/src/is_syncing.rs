use channel::Receiver;
use futures::StreamExt;
use logger::prelude::*;
use std::sync::mpsc::SyncSender;

#[derive(Debug)]
pub enum IsSyncingMessage {
    Check,
    Terminate(SyncSender<()>),
}

pub struct IsSyncingChecker {
    rebusd_endpoint: String,
    receiver: Receiver<IsSyncingMessage>,
}

impl IsSyncingChecker {
    pub fn new(rebusd_endpoint: String, receiver: Receiver<IsSyncingMessage>) -> Self {
        Self {
            rebusd_endpoint,
            receiver,
        }
    }
    pub async fn run(mut self) {
        while let Some(message) = self.receiver.next().await {
            match message {
                IsSyncingMessage::Check => {
                    match rebuscli::get_client(&self.rebusd_endpoint)
                        .lock()
                        .await
                        .fetch_syncing()
                        .await
                    {
                        Ok(syncing) => {
                            if syncing {
                                error!(
                                    "the rebus daemon: {} is syncing",
                                    self.rebusd_endpoint.as_str()
                                );
                            } else {
                                info!(
                                    "the rebus daemon: {} is synced",
                                    self.rebusd_endpoint.as_str()
                                );
                            }
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                }
                IsSyncingMessage::Terminate(sender) => {
                    info!("is syncing checker will be terminated soon...");
                    let _ = sender.send(());
                    break;
                }
            }
        }
    }
}
