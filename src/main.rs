mod app_config;

fn main() -> Result<(), confy::ConfyError> {
    let cfg: app_config::AppConfig = confy::load("cmc", None)?;
    dbg!(cfg);

    Ok(())
}
