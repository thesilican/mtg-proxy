use anyhow::Result;
use log::LevelFilter;
use mtg_proxy::{Printer, DEFAULT_OPTIONS};
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

    let printer = Printer::new();
    let cards = &[
        Uuid::parse_str("e01a59e7-bde1-4150-bb4f-a19d769764f2")?,
        Uuid::parse_str("e01a59e7-bde1-4150-bb4f-a19d769764f2")?,
        Uuid::parse_str("e01a59e7-bde1-4150-bb4f-a19d769764f2")?,
        Uuid::parse_str("e01a59e7-bde1-4150-bb4f-a19d769764f2")?,
        Uuid::parse_str("e01a59e7-bde1-4150-bb4f-a19d769764f2")?,
        Uuid::parse_str("e01a59e7-bde1-4150-bb4f-a19d769764f2")?,
        Uuid::parse_str("98c1b465-b6d9-491b-bfc2-c034cc825d27")?,
        Uuid::parse_str("98c1b465-b6d9-491b-bfc2-c034cc825d27")?,
        Uuid::parse_str("98c1b465-b6d9-491b-bfc2-c034cc825d27")?,
        Uuid::parse_str("98c1b465-b6d9-491b-bfc2-c034cc825d27")?,
        Uuid::parse_str("98c1b465-b6d9-491b-bfc2-c034cc825d27")?,
        Uuid::parse_str("98c1b465-b6d9-491b-bfc2-c034cc825d27")?,
        Uuid::parse_str("349ac7e6-af38-4dc3-abfe-369564c75630")?,
        Uuid::parse_str("349ac7e6-af38-4dc3-abfe-369564c75630")?,
        Uuid::parse_str("349ac7e6-af38-4dc3-abfe-369564c75630")?,
        Uuid::parse_str("349ac7e6-af38-4dc3-abfe-369564c75630")?,
        Uuid::parse_str("349ac7e6-af38-4dc3-abfe-369564c75630")?,
        Uuid::parse_str("349ac7e6-af38-4dc3-abfe-369564c75630")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("804458a2-5376-462d-a2cd-fa596750c0aa")?,
        Uuid::parse_str("8ef23258-511f-43f8-b84f-bd2256b5c86b")?,
        Uuid::parse_str("8ef23258-511f-43f8-b84f-bd2256b5c86b")?,
        Uuid::parse_str("8ef23258-511f-43f8-b84f-bd2256b5c86b")?,
        Uuid::parse_str("8ef23258-511f-43f8-b84f-bd2256b5c86b")?,
        Uuid::parse_str("8ef23258-511f-43f8-b84f-bd2256b5c86b")?,
        Uuid::parse_str("8ef23258-511f-43f8-b84f-bd2256b5c86b")?,
        Uuid::parse_str("8ef23258-511f-43f8-b84f-bd2256b5c86b")?,
        Uuid::parse_str("8ef23258-511f-43f8-b84f-bd2256b5c86b")?,
        Uuid::parse_str("0f7f1148-7b1b-4969-a2f8-428de1e2e8ff")?,
        Uuid::parse_str("0f7f1148-7b1b-4969-a2f8-428de1e2e8ff")?,
        Uuid::parse_str("b02452bb-a049-4e86-ba2c-135803caa03d")?,
        Uuid::parse_str("3d2bd7b4-28de-4d9e-86c5-a46bd608cb02")?,
        Uuid::parse_str("3d2bd7b4-28de-4d9e-86c5-a46bd608cb02")?,
        Uuid::parse_str("3d2bd7b4-28de-4d9e-86c5-a46bd608cb02")?,
        Uuid::parse_str("3d2bd7b4-28de-4d9e-86c5-a46bd608cb02")?,
        Uuid::parse_str("3d2bd7b4-28de-4d9e-86c5-a46bd608cb02")?,
        Uuid::parse_str("3d2bd7b4-28de-4d9e-86c5-a46bd608cb02")?,
        Uuid::parse_str("0a8b9d37-e89c-44ad-bd1b-51cb06ec3e0b")?,
        Uuid::parse_str("0a8b9d37-e89c-44ad-bd1b-51cb06ec3e0b")?,
        Uuid::parse_str("29479ab2-c492-479c-82ba-441703b27c0c")?,
        Uuid::parse_str("29479ab2-c492-479c-82ba-441703b27c0c")?,
        Uuid::parse_str("29479ab2-c492-479c-82ba-441703b27c0c")?,
        Uuid::parse_str("29479ab2-c492-479c-82ba-441703b27c0c")?,
        Uuid::parse_str("29479ab2-c492-479c-82ba-441703b27c0c")?,
        Uuid::parse_str("29479ab2-c492-479c-82ba-441703b27c0c")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("20aff4af-5128-432f-a8c8-65b6909d31ac")?,
        Uuid::parse_str("170e792c-80d5-4775-ad95-37614574ab84")?,
        Uuid::parse_str("170e792c-80d5-4775-ad95-37614574ab84")?,
        Uuid::parse_str("170e792c-80d5-4775-ad95-37614574ab84")?,
        Uuid::parse_str("170e792c-80d5-4775-ad95-37614574ab84")?,
        Uuid::parse_str("170e792c-80d5-4775-ad95-37614574ab84")?,
        Uuid::parse_str("170e792c-80d5-4775-ad95-37614574ab84")?,
        Uuid::parse_str("170e792c-80d5-4775-ad95-37614574ab84")?,
        Uuid::parse_str("170e792c-80d5-4775-ad95-37614574ab84")?,
    ];
    let bytes = printer.print(cards, &DEFAULT_OPTIONS).await?;
    File::create("out.pdf").await?.write_all(&bytes).await?;
    Ok(())
}
