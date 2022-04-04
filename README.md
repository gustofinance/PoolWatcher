# Pool Watcher : a crate that keep pools data updated

Create a PoolWatcher object by passing the pools to monitor.
PoolWatcher emits a `PoolEvent::UpdatedPool` at each new block, containing the pools datas.
The datas represent the assets that compose the pool and the total_share.

``` json
{
    "assets": [
        {
            "info": {
                "token": {
                    "contract_addr": "terra178v546c407pdnx5rer3hu8s2c0fc924k74ymnn"
                }
            },
            "amount": "22062562"
        },
        {
            "info": {
                "token": {
                    "contract_addr": "terra12897djskt9rge8dtmm86w654g7kzckkd698608"
                }
            },
            "amount": "1459047478072"
        }
    ],
    "total_share": "5347439739"
}
```



### Example

``` rust
let watcher = PoolWatcher::new(
        "ws://172.16.0.1:26657/websocket",
        "http://lcd.terra.dev",
        &["terra14zhkur7l7ut7tx6kvj28fp5q982lrqns59mnp3".to_string()],
    );

    if let Ok(mut receiver) = watcher.start().await {
        loop {
            if let Some(event) = receiver.recv().await {
                match event {
                    PoolEvent::UpdatedPool(pools) => {
                        println!("{} pool(s) updated", pools.len());

                        for (pool_address, data) in pools {
                            println!("{} : {:?}", pool_address, data.assets);
                        }
                    }
                }
            }
        }
    }
```

### TODO

- [ ] Add `stop` function



inspired by [twelvepool](https://github.com/setten-io/twelvepool)