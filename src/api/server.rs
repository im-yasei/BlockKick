use crate::api::mining;
use crate::api::queries;
use crate::api::transactions;
use std::sync::{Arc, Mutex};
use std::thread;
use tiny_http::{Request, Response, Server, StatusCode};

use crate::mempool::Mempool;
use crate::storage::Blockchain;

// Контекст для передачи в хендлеры
pub struct ApiContext {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub mempool: Arc<Mutex<Mempool>>,
}

pub fn start_server(ctx: ApiContext, port: u16) -> Result<(), String> {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).map_err(|e| format!("Failed to start server: {}", e))?;

    println!("BlockKick Node API running on http://{}", addr);

    let ctx = Arc::new(ctx);

    for request in server.incoming_requests() {
        let ctx_clone = Arc::clone(&ctx);

        thread::spawn(move || {
            if let Err(e) = handle_request(request, &ctx_clone) {
                eprintln!("Error handling request: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_request(request: Request, ctx: &Arc<ApiContext>) -> Result<(), String> {
    let url = request.url().to_string();
    let method = request.method().to_string();

    let path = url.split("?").next().unwrap_or(&url);

    match (method.as_str(), path) {
        // === Transactions ===
        ("POST", u) if u.starts_with("/api/v1/transactions") && !u.contains("/transactions/") => {
            transactions::handle_post(request, &ctx.blockchain, &ctx.mempool)
        }
        ("GET", u) if u.starts_with("/api/v1/transactions/") => {
            transactions::handle_get(request, &ctx.blockchain, &ctx.mempool)
        }

        // === Mining ===
        ("GET", "/api/v1/mining/candidate") => {
            mining::handle_get_candidate(request, &ctx.blockchain, &ctx.mempool)
        }
        ("POST", "/api/v1/mining/submit") => mining::handle_submit(request, &ctx.blockchain),

        // === Queries ===
        ("GET", u) if u.starts_with("/api/v1/balance/") => {
            queries::handle_balance(request, &ctx.blockchain)
        }
        ("GET", "/api/v1/chain") => queries::handle_chain_info(request, &ctx.blockchain),
        ("GET", u) if u.starts_with("/api/v1/block/") => {
            queries::handle_block(request, &ctx.blockchain)
        }
        ("GET", "/api/v1/projects") => queries::handle_projects(request, &ctx.blockchain),

        // === Unknown ===
        _ => {
            let response =
                Response::new(StatusCode(404), vec![], b"Not Found".as_slice(), None, None);
            request.respond(response).map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
