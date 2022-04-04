use std::collections::HashMap;

use crate::{
    error::PoolWatcherError,
    pool::{PoolData, PoolManager},
};
use log::{debug, error, trace};
use serde_json::Value;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use websocket::{ClientBuilder, OwnedMessage};
const SUBSCRIBE_MSG: &str = r#"{"jsonrpc": "2.0","method": "subscribe","id": 0,"params": {"query": "tm.event='NewBlockHeader'"}}"#;

#[derive(Debug)]
pub enum PoolEvent {
    UpdatedPool(HashMap<String, PoolData>),
}
pub struct PoolWatcher {
    websocket_address: String,
    pool_manager: PoolManager,
    pools: Vec<String>,
}

impl PoolWatcher {
    /// Returns a PoolWatcher
    ///
    /// # Arguments
    ///
    /// * `websocket_address` - websocket url to the terra node (ex: ws://172.16.0.1:26657/websocket)
    /// * `chain_url` - URL to the terra LCD (ex: http://lcd.terra.dev)
    /// * `pools` - String list that represent the pools contract to keep updated ex (&["terraXXX","terraYYY"])
    pub fn new(websocket_address: &str, chain_url: &str, pools: &[String]) -> Self {
        PoolWatcher {
            websocket_address: websocket_address.into(),
            pool_manager: PoolManager {
                chain_url: chain_url.to_string(),
            },
            pools: pools.to_vec(),
        }
    }

    /// Get the pool data at each new block and send the PoolEvent::UpdatedPool message with the updated pools
    pub async fn start(&self) -> Result<UnboundedReceiver<PoolEvent>, PoolWatcherError> {
        let (sender, reader) = unbounded_channel::<PoolEvent>();
        let (mut websocket_reader, mut websocket_writer) = self.create_newblock_websocket()?;
        let pool_manager = self.pool_manager.clone();
        let pools = self.pools.clone();

        tokio::spawn(async move {
            loop {
                let message = websocket_reader
                    .recv_message()
                    .unwrap_or_else(|_| OwnedMessage::Text("".to_string()));
                match message {
                    OwnedMessage::Text(value) => {
                        if let Ok(val) = serde_json::from_str::<Value>(&value) {
                            if let Some(height) =
                                val["result"]["data"]["value"]["header"]["height"].as_str()
                            {
                                debug!("block height: {}", height);

                                match pool_manager.update_pools(&pools[..]).await {
                                    Ok(pools) => {
                                        debug!("send event");
                                        if let Err(error) =
                                            sender.send(PoolEvent::UpdatedPool(pools))
                                        {
                                            error!("{}", error);
                                        }
                                    }
                                    Err(error) => {
                                        error!("{:?}", error)
                                    }
                                }
                            }
                        }
                    }
                    OwnedMessage::Ping(ping) => {
                        debug!("ping");
                        websocket_writer
                            .send_message(&OwnedMessage::Pong(ping))
                            .unwrap();
                    }
                    OwnedMessage::Close(_) => {
                        debug!("close message received.");
                        break;
                    }
                    _ => {
                        trace!("unhandled message received");
                    }
                }
            }
        });

        Ok(reader)
    }

    fn create_newblock_websocket(
        &self,
    ) -> Result<
        (
            websocket::receiver::Reader<std::net::TcpStream>,
            websocket::sender::Writer<std::net::TcpStream>,
        ),
        PoolWatcherError,
    > {
        match ClientBuilder::new(&self.websocket_address) {
            Ok(mut builder) => match builder.connect_insecure() {
                Ok(mut stream) => {
                    stream.send_message(&OwnedMessage::Text(SUBSCRIBE_MSG.into()))?;
                    stream.split().map_err(|err| err.into())
                }
                Err(error) => Err(error.into()),
            },
            Err(error) => {
                error!("{}", error);
                Err(error.into())
            }
        }
    }
}
