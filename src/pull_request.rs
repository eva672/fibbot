use octocrab::{models::repos::DiffEntry, Page};
use std::env;

pub async fn get_pr() -> Result<Page<DiffEntry>, octocrab::Error> {
  let pr_number: u64 = env::var("PR_NUMBER")
    .expect("couldn't get pr_number")
    .parse::<u64>()
    .expect("invalid value");
    octocrab::instance().pulls("eva672", "fibbot").list_files(pr_number).await
  }
