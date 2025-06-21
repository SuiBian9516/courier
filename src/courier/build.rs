use std::{borrow::Borrow, fmt::Display};

use git2::{Error, Repository};

fn main() {
  let (commit_id, branch_name) = get_git_info().unwrap();
  set_env("COMMIT_ID", commit_id);
  set_env("BRANCH", branch_name);
}

fn get_git_info() -> Result<(String, String), Error> {
  let repo = Repository::open(std::env::current_dir().unwrap().join("../../"))?;
  let head = repo.head()?;

  let commit_id = head.target().ok_or(Error::from_str("Cannot get commit ID"))?;
  let branch_name = head.shorthand().map(|s| s.to_string()).unwrap_or("Unknown".to_string());

  Ok((format!("{}", commit_id), branch_name))
}

fn set_env<V: Display>(key: &str, value: V)
where
  String: Borrow<V>,
{
  println!("cargo:rustc-env={}={}", key, value);
}
