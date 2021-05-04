use eventstore::{ClientSettings, ClientSettingsParseError, ProjectionClient};
use structopt::StructOpt;

static PROJECTION_FILE: &'static str = include_str!("../etc/projection.js");
#[derive(StructOpt, Debug)]
struct Params {
    #[structopt(short = "c",  long = "connection-string", default_value = "esdb://localhost:2113", parse(try_from_str = parse_connection_string))]
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
    let client = ProjectionClient::create(params.conn_setts).await?;

    client
        .create_continuous_projection(proj_name.as_str(), PROJECTION_FILE.to_string(), true, None)
        .await?;

    client.abort_projection(proj_name.as_str(), None).await?;

    client
        .delete_projection(proj_name.as_str(), true, true, true, None)
        .await?;

    let proj_name = name_gen.next().unwrap();

    // The server is likely to send an error at that level.
    client
        .create_continuous_projection(proj_name.as_str(), PROJECTION_FILE.to_string(), true, None)
        .await?;

    Ok(())
}
