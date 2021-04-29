mod config;
mod install_activate;

use holochain::conductor::{
    api::error::ConductorApiResult, manager::handle_shutdown, Conductor, ConductorHandle,
};
use holochain_p2p::kitsune_p2p::dependencies::kitsune_p2p_types::dependencies::observability::{
    self, Output,
};
#[allow(deprecated)]
use holochain_types::{app::InstalledAppId, prelude::CellNick};
use std::path::Path;
use structopt::StructOpt;
use tracing::*;

const SAMPLE_DNA: &'static [u8] = include_bytes!("../dna/sample/sample.dna");

#[derive(Debug, StructOpt)]
#[structopt(name = "embedded-holochain-demo", about = "demo of an embedded holochain app & dna runner")]
struct Opt {
    #[structopt(default_value = "my_app_id")]
    app_id: String,

    #[structopt(default_value = "1234")]
    admin_ws_port: u16,

    #[structopt(default_value = "8888")]
    app_ws_port: u16,

    #[structopt(default_value = "databases")]
    datastore_path: String,

    #[structopt(default_value = "keystore")]
    keystore_path: String,
}

fn main() {
    tokio_helper::block_forever_on(async_main())
}

async fn async_main() {
    // Sets up a human-readable panic message with a request for bug reports
    // See https://docs.rs/human-panic/1.0.3/human_panic/
    human_panic::setup_panic!();
    // take in command line arguments
    let opt = Opt::from_args();
    observability::init_fmt(Output::Log).expect("Failed to start contextual logging");
    debug!("observability initialized");
    // Uncomment this to get regular networking info status updates in the logs
    // kitsune_p2p_types::metrics::init_sys_info_poll();
    if !Path::new(&opt.datastore_path).exists() {
        if let Err(e) = std::fs::create_dir(&opt.datastore_path) {
            error!("{}", e);
            panic!()
        };
    }
    let conductor =
        conductor_handle(opt.admin_ws_port, &opt.datastore_path, &opt.keystore_path).await;
    let _ = install_or_passthrough(&conductor, opt.app_id, opt.app_ws_port).await;
    println!("EMBEDDED_HOLOCHAIN_IS_READY");
    // Await on the main JoinHandle, keeping the process alive until all
    // Conductor activity has ceased
    let result = conductor
        .take_shutdown_handle()
        .await
        .expect("The shutdown handle has already been taken.")
        .await;
    handle_shutdown(result);
}

async fn conductor_handle(
    admin_ws_port: u16,
    databases_path: &str,
    keystore_path: &str,
) -> ConductorHandle {
    let config = config::conductor_config(admin_ws_port, databases_path, keystore_path);
    // Initialize the Conductor
    Conductor::builder()
        .config(config)
        .build()
        .await
        .expect("Could not initialize Conductor from configuration")
}

async fn install_or_passthrough(
    conductor: &ConductorHandle,
    app_id: InstalledAppId,
    app_ws_port: u16,
) -> ConductorApiResult<()> {
    #[allow(deprecated)]
    let dnas: Vec<(Vec<u8>, CellNick)> = vec![(SAMPLE_DNA.into(), "sample".into())];
    let cell_ids = conductor.list_cell_ids().await?;
    if cell_ids.len() == 0 {
        println!("Don't see existing files or identity, so starting fresh...");
        install_activate::install_app(&conductor, app_id.clone(), dnas).await?;
        println!("Installed, now activating...");
        install_activate::activate_app(&conductor, app_id).await?;
        // add a websocket interface on the first run
        // it will boot again at the same interface on second run
        conductor.clone().add_app_interface(app_ws_port).await?;
        println!("APP_WS_PORT: {}", app_ws_port);
        println!("Activated.");
    }
    Ok(())
}
