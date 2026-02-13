use axum::{
    body::Bytes,
    extract::State,
    http::{Method, HeaderMap, StatusCode, Uri},
    response::IntoResponse,
};
use reqwest::Client;

use crate::verification::verify_signature;
use crate::DBG_MODE;

pub async fn proxy_handler(
    State(client): State<Client>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {


    if DBG_MODE {
        dbg!(&method, 
            &uri, 
            &headers, 
            &body
        );
    }

    let path_query = uri.path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("");
    let target_url = format!("http://127.0.0.1:3000{}", path_query);

    // verifiy signatures of the requests
    let signature = headers
        .get("x-fanouni-signature")
        .and_then(|h| h.to_str().ok());

    if  method == Method::POST || 
        method == Method::PUT  || 
        method == Method::GET  {

        match signature {
            Some(sig) if verify_signature(&body, sig) => {
                if DBG_MODE {
                    dbg!(sig);
                    println!("󱎚  Integrity Verified");
                }
            }
            _ => {
                if DBG_MODE {
                    println!("󱙱  Signature mismatch or missing!");
                }
                return (StatusCode::UNAUTHORIZED, "Invalid Integrity Signature")
                    .into_response();
            }
        }
    }

    // Direct to NestJS
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
            (status, headers, body).into_response()
        }
        Err(_) => (StatusCode::BAD_GATEWAY, "NestJS is unreachable").into_response()
    }
}
