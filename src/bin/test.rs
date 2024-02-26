use anyhow::Result;
use log::LevelFilter;
use mtg_proxy::{CardFace, CardOptions, Printer, DEFAULT_PAGE_OPTIONS};
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .format_target(false)
        .parse_default_env()
        .init();

    let cards = &[
        CardOptions {
            id: Uuid::parse_str("e01a59e7-bde1-4150-bb4f-a19d769764f2")?,
            face: CardFace::Front,
            quantity: 4,
        },
        CardOptions {
            id: Uuid::parse_str("98c1b465-b6d9-491b-bfc2-c034cc825d27")?,
            face: CardFace::Front,
            quantity: 4,
        },
        CardOptions {
            id: Uuid::parse_str("349ac7e6-af38-4dc3-abfe-369564c75630")?,
            face: CardFace::Front,
            quantity: 4,
        },
    ];
    let bytes = Printer::print(cards, &DEFAULT_PAGE_OPTIONS).await?;
    File::create("out.pdf").await?.write_all(&bytes).await?;
    Ok(())
}
