mod app_config;
use std::vec;

use app_config::AppConfig;

mod dir_tree;
use dir_tree::DirTree;

fn main() -> Result<(), confy::ConfyError> {
    let cfg: AppConfig = confy::load("cmc", None)?;
    dbg!(&cfg);

    let mut dir_tree = DirTree::from_config(&cfg);

    dir_tree.put_into(
        vec![
        "ATTACK".to_string(),
        "BRIGHT".to_string(),
        "ELECTRO".to_string()],
        "track1.mp3".to_string()
        ).unwrap();

    println!("{}", dir_tree.to_string());

    Ok(())
}
