use axum::{
    body::Bytes,
    http::{Method, HeaderMap, StatusCode},
    response::IntoResponse,
};
use reqwest::Client;

use tracing::{debug, info};
use crate::DBG_MODE;

pub async fn redirect_to_backend(
    method: Method, 
    target_url: String, 
    headers: HeaderMap,
    body: Bytes, 
    client: Client
) -> axum::http::Response<axum::body::Body> {

    let response = client
        .request(method, &target_url)
        .headers(headers)
        .body(body)
        .send()
        .await;

    match response {
        Ok(res) => {
            let status = res.status();
            let headers = res.headers().clone();
            let body = res.bytes().await.unwrap_or_default();

            // deprecated feature
            if DBG_MODE {
                println!("response = {:#?}", &body);
                println!("\nresponse_status = {status:#?}");
                println!("---------- End of the Request ----------\n\n");
            }

            info!("sent successfully to ({})", target_url);
            debug!("{:#?}", &body);
            info!("response_status = {status:#?}");

            (status, headers, body).into_response()
        }
        Err(res) => {
            // deprecated feature
            if DBG_MODE {
                println!("response = {:#?}", res);
                println!("\nresponse_status = {:#?}", StatusCode::BAD_GATEWAY);
                println!("---------- End of the Request ----------\n\n");
            }

            info!("{:#?}", res.to_string());
            info!("{:#?}", StatusCode::BAD_GATEWAY);

            (StatusCode::BAD_GATEWAY, "NestJS is unreachable").into_response()
        }
    }
}
