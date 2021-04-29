mod embedded_runner;

#[allow(deprecated)]
use holochain_types::prelude::CellNick;
use structopt::StructOpt;

const SAMPLE_DNA: &'static [u8] = include_bytes!("../dna/sample/sample.dna");

#[derive(Debug, StructOpt)]
#[structopt(
    name = "embedded-holochain-demo",
    about = "demo of an embedded holochain app & dna runner"
)]
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
    let opt = Opt::from_args();
    #[allow(deprecated)]
    let dnas: Vec<(Vec<u8>, CellNick)> = vec![(SAMPLE_DNA.into(), "sample".into())];
    embedded_runner::async_main(embedded_runner::HcConfig {
        app_id: opt.app_id,
        dnas,
        admin_ws_port: opt.admin_ws_port,
        app_ws_port: opt.app_ws_port,
        datastore_path: opt.datastore_path,
        keystore_path: opt.keystore_path,
    })
}
