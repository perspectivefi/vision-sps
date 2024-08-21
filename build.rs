use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ERC4626", "abi/ERC4626.json")?
        .generate()?
        .write_to_file("src/abi/erc4626.rs")?;

    Ok(())
}