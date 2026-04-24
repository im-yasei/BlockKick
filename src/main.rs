use std::env;
use std::process;
use std::sync::{Arc, Mutex};

mod api;
mod consensus;
mod crypto;
mod mempool;
mod state;
mod storage;
mod types;
mod validator;

use api::server::{ApiContext, start_server};
use mempool::mempool::Mempool;
use storage::blockchain::Blockchain;
use types::Block;

/// Конфигурация ноды
struct NodeConfig {
    port: u16,
    difficulty: u32,
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            port: 3000,
            difficulty: 4,
        }
    }
}

fn main() {
    println!("BlockKick Node v0.3.0");
    println!("========================");

    // Парсим аргументы командной строки
    let config = parse_args();

    println!("Configuration:");
    println!("   Port: {}", config.port);
    println!("   Difficulty: {}", config.difficulty);
    println!();

    // Инициализация блокчейна
    println!("Initializing blockchain...");
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // Проверяем что genesis блок создан
    {
        let chain = blockchain.lock().unwrap();
        println!(
            "   Genesis block hash: {}...",
            &chain.get_latest_block().unwrap().calculate_hash()[..16]
        );
        println!("   Chain height: {}", chain.height());
    }

    // Инициализация мемпула
    println!("Initializing mempool...");
    let mempool = Arc::new(Mutex::new(Mempool::new()));

    // Создаём контекст для API
    let ctx = ApiContext {
        blockchain: Arc::clone(&blockchain),
        mempool: Arc::clone(&mempool),
    };

    // Запуск HTTP сервера
    println!("Starting API server on http://0.0.0.0:{}...", config.port);
    println!();
    println!("Available endpoints:");
    println!("   GET  /api/v1/chain              - Chain info");
    println!("   GET  /api/v1/balance/:address   - Get balance");
    println!("   GET  /api/v1/block/:height      - Get block by height");
    println!("   GET  /api/v1/projects           - List projects");
    println!("   GET  /api/v1/transactions/:id   - Transaction status");
    println!("   POST /api/v1/transactions       - Submit transaction");
    println!("   GET  /api/v1/mining/candidate   - Get mining template");
    println!("   POST /api/v1/mining/submit      - Submit mined block");
    println!();
    println!("Press Ctrl+C to stop the node");
    println!("========================");

    // Запускаем сервер (блокирует основной поток)
    if let Err(e) = start_server(ctx, config.port) {
        eprintln!("Server error: {}", e);
        process::exit(1);
    }
}

/// Парсит аргументы командной строки
fn parse_args() -> NodeConfig {
    let mut config = NodeConfig::default();

    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" | "-p" => {
                if i + 1 < args.len() {
                    if let Ok(port) = args[i + 1].parse::<u16>() {
                        config.port = port;
                        i += 1;
                    } else {
                        eprintln!(" Invalid port number, using default 3000");
                    }
                }
            }
            "--difficulty" | "-d" => {
                if i + 1 < args.len() {
                    if let Ok(diff) = args[i + 1].parse::<u32>() {
                        config.difficulty = diff;
                        i += 1;
                    } else {
                        eprintln!(" Invalid difficulty, using default 4");
                    }
                }
            }
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            _ => {
                eprintln!(" Unknown argument: {}", args[i]);
            }
        }
        i += 1;
    }

    config
}

/// Выводит справку по использованию
fn print_help() {
    println!("BlockKick Node - Децентрализованная краудфандинговая платформа");
    println!();
    println!("USAGE:");
    println!("    cargo run [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -p, --port <PORT>              HTTP port (default: 3000)");
    println!("    -d, --difficulty <DIFFICULTY>  PoW difficulty (default: 4)");
    println!("    -h, --help                     Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    cargo run                      # Start on port 3000");
    println!("    cargo run -- --port 8080       # Start on port 8080");
    println!("    cargo run -- -p 9000 -d 2      # Port 9000, difficulty 2");
    println!();
    println!("API ENDPOINTS:");
    println!("    http://localhost:3000/api/v1/chain");
    println!("    http://localhost:3000/api/v1/mining/candidate");
    println!("    http://localhost:3000/api/v1/balance/<address>");
}
