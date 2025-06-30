use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::net::SocketAddr;

#[derive(Serialize)]
struct WalletInfo{
    balance_sol:f64,
}

async fn get_wallet_info(Path(address):Path<String>) -> impl IntoResponse {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    //let us parse the address
    let pubkey = match address.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid wallet address").into_response(),
    };
    // fetching the balance in lamoports

    let lamports = match client.get_balance(&pubkey){
        Ok(lamports) => lamports,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("RPC error: {}", err)).into_response(),
    };
    let balance_sol = lamports as f64 / 1_000_000_000.0;

    let response = WalletInfo { balance_sol};
    Json(response).into_response()
}

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/wallet/{address}",get(get_wallet_info));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("Listening on http://{}",addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(),app).await.unwrap();
}