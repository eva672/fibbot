use octocrab::{models::repos::DiffEntry, Page};

pub async fn get_pr(owner: &str, repo: &str) -> Result<Page<DiffEntry>, octocrab::Error> {
    octocrab::instance().pulls(owner, repo).list_files(1).await
  }
