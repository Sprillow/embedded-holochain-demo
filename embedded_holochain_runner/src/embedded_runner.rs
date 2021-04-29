use holochain::conductor::{
    api::error::ConductorApiResult, manager::handle_shutdown, Conductor, ConductorHandle,
};
use holochain_p2p::kitsune_p2p::dependencies::kitsune_p2p_types::dependencies::observability::{
    self, Output,
};
use holochain_types::app::InstalledAppId;
use std::path::Path;
use tracing::*;

pub struct HcConfig {
    pub app_id: String,
    pub dnas: Vec<(Vec<u8>, String)>,
    pub admin_ws_port: u16,
    pub app_ws_port: u16,
    pub datastore_path: String,
    pub keystore_path: String,
    pub proxy_url: String,
}

pub fn async_main(hc_config: HcConfig) {
    tokio_helper::block_forever_on(inner_async_main(hc_config))
}

async fn inner_async_main(hc_config: HcConfig) {
    // Sets up a human-readable panic message with a request for bug reports
    // See https://docs.rs/human-panic/1.0.3/human_panic/
    human_panic::setup_panic!();
    // take in command line arguments
    observability::init_fmt(Output::Log).expect("Failed to start contextual logging");
    debug!("observability initialized");
    // Uncomment this to get regular networking info status updates in the logs
    // kitsune_p2p_types::metrics::init_sys_info_poll();
    if !Path::new(&hc_config.datastore_path).exists() {
        if let Err(e) = std::fs::create_dir(&hc_config.datastore_path) {
            error!("{}", e);
            panic!()
        };
    }
    // run up a conductor
    let conductor = conductor_handle(
        hc_config.admin_ws_port,
        &hc_config.datastore_path,
        &hc_config.keystore_path,
        &hc_config.proxy_url,
    )
    .await;
    // install the app with its dnas, if they aren't already
    // as well as adding the app_ws_port
    let _ = install_or_passthrough(
        &conductor,
        hc_config.app_id,
        hc_config.app_ws_port,
        hc_config.dnas,
    )
    .await;
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
    proxy_url: &str,
) -> ConductorHandle {
    let config =
        super::config::conductor_config(admin_ws_port, databases_path, keystore_path, proxy_url);
    // Initialize the Conductor
    Conductor::builder()
        .config(config)
        .build()
        .await
        .expect("Could not initialize Conductor from configuration")
}

#[allow(deprecated)]
async fn install_or_passthrough(
    conductor: &ConductorHandle,
    app_id: InstalledAppId,
    app_ws_port: u16,
    dnas: Vec<(Vec<u8>, String)>,
) -> ConductorApiResult<()> {
    #[allow(deprecated)]
    let cell_ids = conductor.list_cell_ids().await?;
    if cell_ids.len() == 0 {
        println!("Don't see existing files or identity, so starting fresh...");
        super::install_activate::install_app(&conductor, app_id.clone(), dnas).await?;
        println!("Installed, now activating...");
        super::install_activate::activate_app(&conductor, app_id).await?;
        // add a websocket interface on the first run
        // it will boot again at the same interface on second run
        conductor.clone().add_app_interface(app_ws_port).await?;
        println!("APP_WS_PORT: {}", app_ws_port);
        println!("Activated.");
    }
    Ok(())
}
