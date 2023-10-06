fn main() {
    if let Err(e) = git_commit_helper_jp::run() {
        eprintln!("{}", e)
    }
}
