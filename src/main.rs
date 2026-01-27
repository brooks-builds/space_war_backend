use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    space_war_backend::run().await
}
