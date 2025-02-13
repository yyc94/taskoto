pub mod sync {
    use git2::{Repository, IndexAddOption, Signature};
    use crate::*;

    pub fn init_repo() -> Result<Repository, git2::Error> {
        Repository::init(&get_database_dir()) 
    } 

    pub fn sync_push() -> Result<(), git2::Error> {
        let repo = Repository::open(&get_database_dir())?;
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;
    
        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;
    
        let head = repo.head()?.peel_to_commit()?;
        let signature = Signature::now(&get_user_name(), &get_email())?;
        repo.commit(Some("HEAD"), &signature, &signature, "Commit message", &tree, &[&head])?;
        println!("Changes committed!");
        Ok(())
    }
}
