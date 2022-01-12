#[macro_use]
extern crate log;
use eventstore::{ClientSettings, ClientSettingsParseError, ProjectionClient};
use structopt::StructOpt;

static PROJECTION_FILE: &'static str = include_str!("../etc/projection.js");
#[derive(StructOpt, Debug)]
struct Params {
    #[structopt(short = "c",  long = "connection-string", default_value = "esdb://localhost:2113?tls=false", parse(try_from_str = parse_connection_string))]
    conn_setts: ClientSettings,
}

fn parse_connection_string(
    input: &str,
) -> std::result::Result<ClientSettings, ClientSettingsParseError> {
    ClientSettings::parse_str(input)
}

type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> crate::Result<()> {
    pretty_env_logger::init();
    let params = Params::from_args();
    let mut name_gen = names::Generator::default();

    let proj_name = name_gen.next().unwrap();
    let client = ProjectionClient::new(params.conn_setts);

    client
        .create(proj_name.as_str(), PROJECTION_FILE.to_string(), &Default::default())
        .await?;
    info!("complete first projection creation");

    client.abort(proj_name.as_str(), &Default::default()).await?;
    info!("complete projection abort");

    client
        .delete(proj_name.as_str(), &Default::default())
        .await?;
    info!("complete projection delete");

    let proj_name = name_gen.next().unwrap();

    // The server is likely to send an error at that level.
    client
        .create(proj_name.as_str(), PROJECTION_FILE.to_string(), &Default::default())
        .await?;
    info!("complete second projection creation");

    Ok(())
}
