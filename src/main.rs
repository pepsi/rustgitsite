use git2::Repository;
fn main() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let b = repo.head().unwrap();
    println!("{}", b.name().unwrap());
    // println!("{}", repo.statuses(None).unwrap());
    println!("Hello, world!");
}
