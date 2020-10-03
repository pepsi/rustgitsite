use git2::Repository;

fn get_all_commits(start: git2::Commit) -> Vec<git2::Commit> {
    let mut p: Vec<git2::Commit> = vec![start.clone()];
    for parent in start.parents() {
        let x = get_all_commits(parent);
        for y in x {
            p.push(y);
        }
    }
    return p;
}
fn main() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = repo.head().unwrap();
    for commit in get_all_commits(head.peel_to_commit().unwrap()) {
        println!(
            "{} by {} @ {:?}",
            commit.summary().unwrap(),
            commit.author(),
            commit.time().seconds()
        );
    }
    // println!("commits: {:?}", get_all_commits(head.peel_to_commit().unwrap()));
    println!(
        "Latest commit {}",
        head.peel_to_commit().unwrap().summary().unwrap()
    );
    // println!("{}", repo.statuses(None).unwrap());
}
