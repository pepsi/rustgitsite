use git2::Repository;
fn get_prev_commit(c: git2::Commit) -> Vec<git2::Commit>{
    let mut p: Vec<git2::Commit> = vec![];
    println!("c.parent_count {}", c.parent_count());
    if c.parent_count() == 1{
        return p;
    }else{
        for prev_commit in get_prev_commit(c){
            p.push(prev_commit);
        }
        return p;
    }
}
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
    println!("commits: {:?}", get_prev_commit(head.peel_to_commit().unwrap()));
    println!("Latest commit {}", head.peel_to_commit().unwrap().summary().unwrap());
    // println!("{}", repo.statuses(None).unwrap());
}
