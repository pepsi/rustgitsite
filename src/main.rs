use git2::Repository;
fn main() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = repo.head().unwrap();
    let mut revwalk = repo.revwalk().unwrap();
    let base = git2::Sort::REVERSE;
    // let a = head.peel_to_commit().unwrap().parent(i)
    for parent in head.peel_to_commit().unwrap().parents(){
        println!("parent = {:?}", parent);
    }
    revwalk.set_sorting(base).unwrap();
    println!("st sorting to base");
    for walk in revwalk{
        println!("Walked {}", walk.unwrap());
    }
    println!("Latest commit {}", head.peel_to_commit().unwrap().summary().unwrap());
    // println!("{}", repo.statuses(None).unwrap());
}
