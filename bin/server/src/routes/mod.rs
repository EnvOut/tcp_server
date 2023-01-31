use std::collections::HashMap;

use anyhow::Context;

use common_protocol::models::resp::Response;

use crate::errors::ServerResult;
use crate::services::quotes_service::MockedQuotesService;

#[derive(Default)]
pub struct AppRoutes {
    routes: HashMap<String, fn(RequestData) -> ServerResult<Response>>,
}

pub type RequestData = ();

impl AppRoutes {
    pub fn create_route(
        &mut self,
        resource: &str,
        route: fn(RequestData) -> ServerResult<Response>,
    ) {
        self.routes.insert(resource.to_owned(), route);
    }

    pub fn call_router(&self, resource: &str, request: ()) -> ServerResult<Response> {
        let route = self.routes.get(resource).context("expected route")?;
        route(request)
    }
}

pub fn get_quote(_: RequestData) -> ServerResult<Response> {
    Ok(Response::Resource(MockedQuotesService::get_random_quote()?))
}
