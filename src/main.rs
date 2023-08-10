#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        tracing::error!("Please provide sub-commands: pull, push")
    }
    let subcmd = args[1].as_str();

    match subcmd {
        "pull" => match bili_blocklist_sync::pull().await {
            Ok(_) => tracing::info!("OK"),
            Err(e) => tracing::error!("{:#?}", e),
        },
        "push" => match bili_blocklist_sync::push().await {
            Ok(_) => tracing::info!("OK"),
            Err(e) => tracing::error!("{:#?}", e),
        },
        _ => {}
    };

    if cfg!(target_os = "windows") {
        tracing::info!("> Press [Enter] to close terminal <");
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
}
