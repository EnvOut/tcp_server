pub mod request {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub enum Request {
        AskPazzle,
        GetResource(String),
    }
}

pub mod resp {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub enum Response {
        Pazzle(String),
        Resource(String),
        Error(String),
    }
}
