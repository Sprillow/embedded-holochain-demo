use embedded_holochain_runner::*;
use structopt::StructOpt;

const SAMPLE_DNA: &'static [u8] = include_bytes!("../dna/sample/sample.dna");

#[derive(Debug, StructOpt)]
#[structopt(
    name = "embedded-holochain-demo",
    about = "demo of embedding holochain in a binary via embedded-holochain-runner"
)]
struct Opt {
    #[structopt(long, default_value = "my_app_id")]
    app_id: String,

    #[structopt(long, default_value = "1234")]
    admin_ws_port: u16,

    #[structopt(long, default_value = "8888")]
    app_ws_port: u16,

    #[structopt(long, default_value = "databases")]
    datastore_path: String,

    #[structopt(long, default_value = "keystore")]
    keystore_path: String,

    #[structopt(
        long,
        default_value = "kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--"
    )]
    proxy_url: String,
}

fn main() {
    let opt = Opt::from_args();
    // String is like "CellNick"-ish
    let dnas: Vec<(Vec<u8>, String)> = vec![(SAMPLE_DNA.into(), "sample".into())];
    async_main(HcConfig {
        app_id: opt.app_id,
        dnas,
        admin_ws_port: opt.admin_ws_port,
        app_ws_port: opt.app_ws_port,
        datastore_path: opt.datastore_path,
        keystore_path: opt.keystore_path,
        proxy_url: opt.proxy_url,
    })
}
