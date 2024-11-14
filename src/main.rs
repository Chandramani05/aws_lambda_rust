use lambda_runtime::{
    run, service_fn,
    tracing::{self, event},
    Error, LambdaEvent,
};
mod generic_handler;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    greeting: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    message: String,
}

fn marco_polo_greeting(greeeting: &str) -> String {
    if greeeting == "Marco" {
        "Polo".to_string()
    } else {
        "No".to_string()
    }
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // extract some useful info from the request
    let greeting = event.payload.greeting;

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        message: marco_polo_greeting(&greeting),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
