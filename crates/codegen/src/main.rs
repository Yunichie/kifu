fn main() -> Result<(), Box<dyn std::error::Error>> {
    domain::export_all_bindings()?;
    Ok(())
}
