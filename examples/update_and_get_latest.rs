//! Updates the local git registry and extracts the latest most recent changes.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut index = crates_index::GitIndex::new_cargo_default()?;
    println!("Updating index…");
    index.update()?;

    let limit = 10;
    println!("The most recent {limit} changes:\n");
    for change in index.changes()?.take(limit) {
        let change = change?;
        println!(
            "{name} changed in {commit}",
            name = change.crate_name(),
            commit = change.commit_hex()
        );
    }
    Ok(())
}
