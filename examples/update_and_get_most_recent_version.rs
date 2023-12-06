//! Updates the local git registry and extracts the latest most recent changes.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_name = std::env::args()
        .nth(1)
        .ok_or("The first argument must be the name of the crate to get the most recent version of")?;
    let mut index = crates_index::GitIndex::new_cargo_default()?;
    eprintln!("Updating index…");
    index.update()?;

    let krate = index
        .crate_(&crate_name)
        .ok_or_else(|| format!("Crate named '{crate_name}' does not exist in git index"))?;
    println!("most recent   : {}", krate.most_recent_version().version());
    println!(
        "highest normal: {:?}",
        krate.highest_normal_version().map(|v| v.version())
    );
    println!("highest       : {}", krate.highest_version().version());
    println!("earliest      : {}", krate.earliest_version().version());
    Ok(())
}
