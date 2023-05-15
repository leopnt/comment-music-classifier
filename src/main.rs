mod app_config;
use app_config::AppConfig;

mod dir_tree;
use dir_tree::DirTree;

fn main() -> Result<(), confy::ConfyError> {
    let cfg: AppConfig = confy::load("cmc", None)?;
    dbg!(&cfg);

    let dir_tree = DirTree::from_config(&cfg);

    println!("{}", dir_tree.to_string());

    Ok(())
}
