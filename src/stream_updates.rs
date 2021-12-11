use async_stream::stream;
use frankenstein::{Api, GetUpdatesParams, TelegramApi, Update};
use futures::Stream;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::time::sleep;

/// A utility which is implementing a stream of updates from the Telegram API.
#[derive(Debug, Clone)]
pub struct MoranoStream {
    api: Api,
    update_params: GetUpdatesParams,
    queue: VecDeque<Update>,
}

impl MoranoStream {
    pub fn new(api: Api, allowed_updates: Vec<String>) -> Self {
        let mut update_params = GetUpdatesParams::new();
        update_params.set_allowed_updates(Some(allowed_updates));

        MoranoStream {
            api,
            update_params,
            queue: VecDeque::new(),
        }
    }

    pub fn to_stream(mut self) -> impl Stream<Item = Update> {
        stream! {
                loop {
                if let Some(update) = self.queue.pop_front() {
                    yield update;
                }

                match self.api.get_updates(&self.update_params) {
                    Ok(updates) => {
                        for update in updates.result {
                            self.queue.push_back(update);
                        }

                        if self.queue.len() > 0 {
                            let last_update = self.queue.back().unwrap().update_id + 1;
                            self.update_params.set_offset(Some(last_update));
                            yield self.queue.pop_front().unwrap();
                        }
                    }

                    Err(err) => {
                        println!("Failed to fetch updates {:?}", err);
                    }
                }

                sleep(Duration::from_millis(500)).await;
            }
        }
    }
}
