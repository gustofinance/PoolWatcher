use std::collections::HashMap;

use crate::error;
use log::{debug, error};
use serde_json::Value;

pub const POOL_QUERY: &str = "eyJwb29sIjp7fX0=";

#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct PoolData {
    /// assets (info & amount) in the pool
    pub assets: Vec<Value>,

    /// total share of the pool
    pub total_share: String,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct QueryPoolResponse {
    query_result: PoolData,
}
#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct PoolManager {
    pub chain_url: String,
}

impl PoolManager {
    pub async fn update_pools(
        &self,
        pools_address: &[String],
    ) -> Result<HashMap<String, PoolData>, error::PoolWatcherError> {
        let mut pools: HashMap<String, PoolData> = HashMap::new();
        debug!("{} pool(s) to update", pools_address.len());
        for pool_contract in pools_address {
            debug!("updating {}...", pool_contract);
            match reqwest::get(format!(
                "{}/terra/wasm/v1beta1/contracts/{}/store?query_msg={}",
                self.chain_url, pool_contract, POOL_QUERY
            ))
            .await?
            .json::<QueryPoolResponse>()
            .await
            {
                Ok(query_response) => {
                    debug!("updating {}...done", pool_contract);

                    pools.insert(pool_contract.to_string(), query_response.query_result);
                }

                Err(error) => {
                    error!("{}", error)
                }
            }
        }
        Ok(pools)
    }
}

#[cfg(test)]
mod tests {
    use super::PoolManager;

    #[tokio::test]
    async fn test_update_pools() {
        let pool_manager = PoolManager {
            chain_url: "https://lcd.terra.dev".to_string(),
        };
        // terra14zhkur7l7ut7tx6kvj28fp5q982lrqns59mnp3 is regular pool
        fetch_pool(
            &pool_manager,
            "terra14zhkur7l7ut7tx6kvj28fp5q982lrqns59mnp3",
        )
        .await;

        // terra19d2alknajcngdezrdhq40h6362k92kz23sz62u is prism pool
        fetch_pool(
            &pool_manager,
            "terra19d2alknajcngdezrdhq40h6362k92kz23sz62u",
        )
        .await;
    }

    async fn fetch_pool(pool_manager: &PoolManager, pool_address: &str) {
        let res = pool_manager.update_pools(&[pool_address.to_string()]).await;
        assert!(res.is_ok());
        let pools = res.unwrap();
        assert!(pools.len() == 1);
        let pool = pools.get(pool_address).unwrap();
        assert!(pool.assets.len() == 2);
    }
}
