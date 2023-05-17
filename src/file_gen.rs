use eyre::Result as EyreResult;

pub fn rust_file_generation() -> EyreResult<()> {
    let abi_source = "./abi/ERC20Votes.json";
    let out_file = std::env::current_dir().unwrap().join("src/erc20votes.rs");
    if out_file.exists() {
        std::fs::remove_file(&out_file)?;
    }
    Abigen::new("ERC20Votes", abi_source)?
        .generate()?
        .write_to_file(out_file)?;
    Ok(())
}
