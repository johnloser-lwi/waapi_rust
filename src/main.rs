use std::error::Error;
use futures::future::Map;
use serde::{Deserialize, Serialize};

use wamp_async::{
    try_into_any_value, Client, ClientConfig, ClientRole, SerializerType, WampKwArgs,
};

mod libs {
    pub mod ak_wwise_core_getProjectInfo;
    pub mod ak_wwise_ui_getSelectedObjects;
}

use libs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the server
    let (mut client, (evt_loop, _rpc_evt_queue)) = Client::connect(
        "ws://localhost:8080/waapi",
        Some(
            ClientConfig::default()
                .set_ssl_verify(false)
                // Restrict our roles
                .set_roles(vec![ClientRole::Caller])
                // Only use Json serialization
                .set_serializers(vec![SerializerType::Json]),
        ),
    )
    .await?;
    println!("Connected !!");


    // Spawn the event loop
    tokio::spawn(evt_loop);

    println!("Joining realm");
    client.join_realm("realm1").await?;

    let mut kwd_args = WampKwArgs::new();

    match client.call("ak.wwise.core.getProjectInfo", None, None).await {
        Ok((res_args, res_kwargs)) => {
            match res_kwargs {
                Some(map) => {
                    let string = serde_json::to_string(&map).unwrap();
                    let data: ak_wwise_core_getProjectInfo::Result = serde_json::from_str(string.as_str()).unwrap();
                    println!("{}", data.directories.soundBankOutputRoot);
                    println!("{}", data.displayTitle);
                },
                None => println!("failed to get result")
            }
        }
        Err(e) => {
            println!("Error calling ({:?})", e);
        }
    };

    match client.call("ak.wwise.ui.getSelectedObjects", None, None).await {
        Ok((res_args, res_kwargs)) => {
            match res_kwargs {
                Some(map) => {
                    let string = serde_json::to_string(&map).unwrap();
                    let data: ak_wwise_ui_getSelectedObjects::Result = serde_json::from_str(string.as_str()).unwrap();
                
                    for obj in data.objects {
                        println!("{}: {}", obj.name, obj.id);
                    }
                
                },
                None => println!("failed to get result")
            }
        }
        Err(e) => {
            println!("Error calling ({:?})", e);
        }
    };


    client.leave_realm().await?;
    client.disconnect().await;
    Ok(())
}