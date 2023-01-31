use std::sync::Arc;

use log;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, TcpStream};

use common_pow::verifier::verify_answer_hash;
use common_pow::HashCash;
use common_protocol::models::request::Request;
use common_protocol::models::resp::Response;

use crate::errors::{ServerErrors, ServerResult};
use crate::routes::AppRoutes;
use crate::services::hashcash_service::HardcodedHashcashService;

const VALID_METHOD: &str = "isPrime";

pub struct Server {
    routes: AppRoutes,
    server_address: String,
}

impl Server {
    pub fn new(server_address: String, routes: AppRoutes) -> Self {
        Self {
            routes,
            server_address,
        }
    }

    async fn handle_request(&self, req: Request) -> ServerResult<Response> {
        match req {
            Request::AskPazzle => Ok(Response::Pazzle(
                HardcodedHashcashService::generate_pazzle()?
            )),
            Request::GetResource(pazzle_answer) => {
                let (has_beautiful_hash, prefix, hash) = verify_answer_hash(&pazzle_answer)?;
                if has_beautiful_hash {
                    let hashcash = HashCash::try_from(pazzle_answer.as_str())?;
                    let is_valid = HardcodedHashcashService::validate(&hashcash)?;
                    if is_valid {
                        self.routes.call_router(&hashcash.resource, ())
                    } else {
                        Err(ServerErrors::HashcashInvalid)
                    }
                } else {
                    Err(ServerErrors::IncorrectAnswer(prefix, hash, pazzle_answer))
                }
            }
        }
    }

    async fn handle_connection(
        &self,
        mut stream: TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf_stream = BufStream::new(stream);
        loop {
            let mut bytes_line = vec![];

            bytes_line.clear();
            if let Err(e) = buf_stream.read_until(b'\n', &mut bytes_line).await {
                println!("Unhandled error occured: {}", e);
                break;
            }

            bytes_line.pop();
            let json = String::from_utf8(bytes_line.clone())?;
            let req: Request = serde_json::from_str(&json)?;

            let resp_res = self.handle_request(req).await;

            let resp = match resp_res {
                Ok(resp) => resp,
                Err(err) => Response::Error(err.to_string()),
            };
            let resp_bytes = {
                let mut bytes = serde_json::to_vec(&resp)?;
                bytes
            };

            buf_stream.write_all(&resp_bytes).await?;
            buf_stream.flush().await?;
            break;
        }
        Ok(())
    }
}

pub async fn run(server: Server) -> ServerResult<()> {
    log::info!("Listening on address: {}", &server.server_address);

    let listener = TcpListener::bind(&server.server_address).await?;

    let server = Arc::new(server);
    loop {
        let server = server.clone();
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let result = server.handle_connection(socket).await;

            if let Err(err) = result {
                log::error!("err: {:?}", err);
            }
        });
    }
}

mod tests {
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};

    use common_pow::HashCash;

    use crate::models::ExtStruct;

    #[test]
    fn rr1() {
        let hashcash_str = "1:4:1675198752:quotes:d/YYMaVOOuzJyJAGbYyqXdDgDJ5br71B1gZYOuUpyfnegl2V646ZNLgf1vQp4qdh::8070450532247941494";
        let hashcash = HashCash::try_from(hashcash_str).unwrap();
        println!("hashcash: {:?}", hashcash);
    }
    #[test]
    fn rr() {
        // let json = r###"{"GetResource":"1:4:1675197642:quotes:d/YYMaVOOuzJyJAGbYyqXTis9UaI60GwvzJ0OXX4TALv/GPGrJ7rlOk9VukNb+V+::6917529027641086565"}"###;
        // let req: Request = serde_json::from_str(json).unwrap();

        // if let Request::GetResource(answer) = req {
        //     println!("{}", answer);
        //     let hashcash = HashCash::try_from(answer.as_str()).unwrap();
        //     let is_valid = HardcodedHashcashService::validate(&hashcash).unwrap();
        //     // if is_valid {
        //     //     self.routes.call_router(&hashcash.resource, ())
        //     // } else {
        //     //     Err(ServerErrors::HashcashInvalid)
        //     // }
        //     assert!(is_valid);
        // } else {
        //     assert!(false);
        // }
        const SECRET_KEY: &str = "Q9tV!MtHaNUCAUg4";
        let mc = new_magic_crypt!(SECRET_KEY, 256);
        let ext = "d/YYMaVOOuzJyJAGbYyqXX700lBCMfqtF5+XCMobkKeR8fthmdtlBFieWvgdF6k4";
        let ext_json = mc.decrypt_base64_to_string(ext).unwrap();
        let ext: ExtStruct = serde_json::from_str(&ext_json).unwrap();

        println!("ext: {:?}", ext);
        // hashcash.ext
        // Ok(ext.bits == hashcash.bits && ext.timestamp.timestamp() == (hashcash.timestamp as i64))
    }
}

// d/YYMaVOOuzJyJAGbYyqXX700lBCMfqtF5+XCMobkKeR8fthmdtlBFieWvgdF6k4
// d/YYMaVOOuzJyJAGbYyqXX700lBCMfqtF5+XCMobkKeR8fthmdtlBFieWvgdF6k4
