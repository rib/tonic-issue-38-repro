
pub mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}
pub mod openmatch {
    tonic::include_proto!("openmatch");
}

use openmatch::{
    client::{BackendClient},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    const OM_NAMESPACE: &str = "open-match";
    const OM_BACKEND_HOSTNAME: &str = "om-backend";
    const OM_BACKEND_PORT: u32 = 50505;

    let om_address = format!("http://{}.{}:{}", OM_BACKEND_HOSTNAME, OM_NAMESPACE, OM_BACKEND_PORT);

    let mut _client = match BackendClient::connect(om_address).await {
        Ok(client) => {
            println!("Connected to OpenMatch backend");
            client
        }
        Err(e) => {
            eprintln!("Failed to connect to OpenMatch backend: {:?}", e);
            Err(e)?
        }
    };

    Ok(())
}
