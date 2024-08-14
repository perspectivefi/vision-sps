use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ERC4626", "abi/ERC4626.json")?
        .generate()?
        .write_to_file("src/abi/erc4626.rs")?;

    Abigen::new("CurvePool", "abi/CurvePool.json")?
        .generate()?
        .write_to_file("src/abi/curve_pool.rs")?;

    Abigen::new("Factory", "abi/Factory.json")?
        .generate()?
        .write_to_file("src/abi/factory.rs")?;

    Ok(())
}