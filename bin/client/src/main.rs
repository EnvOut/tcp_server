use std::error::Error;

use anyhow::{anyhow, Context};
use futures::{Sink, SinkExt, Stream, StreamExt};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

use common_pow::parser::verify_pazzle_and_parse_bits;
use common_protocol::models::request::Request;
use common_protocol::models::resp::Response;

use crate::errors::ClientResult;
use crate::prover::find_proof;
use dotenv::dotenv;

pub mod errors;
pub mod prover;

#[tokio::main]
async fn main() -> ClientResult<()> {
    dotenv().ok();
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());
    log::info!("SERVER_ADDRESS: {:?}", server_address);

    let resp = simple_client::send_message(&server_address, Request::AskPazzle).await?;

    let pazzle = match resp {
        Response::Pazzle(pazzle) => Ok(pazzle),
        _ => Err(anyhow!("Unexpecteble response {:?}", resp)),
    }?;

    let bits = verify_pazzle_and_parse_bits(pazzle.as_str())?;
    let (answer, counter, hash) = find_proof(pazzle, bits)?
        .context("can't find proof based on the range from 0 to u64::Max")?;

    let resp = simple_client::send_message(&server_address, Request::GetResource(answer)).await?;
    let pazzle = match resp {
        Response::Resource(resource_data) => Ok(resource_data),
        _ => Err(anyhow!("Unexpecteble response {:?}", resp)),
    }?;

    log::info!("Received: {:?}", pazzle);

    Ok(())
}

pub mod simple_client {
    use futures::TryFutureExt;
    use tokio::io::AsyncReadExt;
    use tokio::io::{AsyncWriteExt, BufStream};
    use tokio::net::TcpStream;

    use common_protocol::models::request::Request;
    use common_protocol::models::resp::Response;

    use crate::errors::ClientResult;

    pub async fn send_message(server: &str, req: Request) -> ClientResult<Response> {
        let req_bytes = {
            let mut json = serde_json::to_string(&req)?;
            let mut bytes = json.into_bytes();
            bytes.push(b'\n');
            bytes
        };

        let mut stream: TcpStream = TcpStream::connect(server)
            .map_err(|e| anyhow::Error::msg(e))
            .await?;
        let mut buf_stream = BufStream::new(stream);
        log::info!("Successfully connected to server {:?}", server);

        buf_stream.write_all(&req_bytes).await?;
        buf_stream.flush().await;
        log::info!("Sent message: {:?}", req);

        let mut data = vec![];
        let _ = buf_stream.read_to_end(&mut data).await?;

        let resp = serde_json::from_slice(&data)?;

        buf_stream.flush().await.unwrap();
        Ok(resp)
    }
}
