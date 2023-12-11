extern crate git2;

use dotenvy_macro::dotenv;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{
    BranchType, Cred, Error, FetchOptions, Object, PushOptions, RemoteCallbacks, Repository,
    Signature,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

pub async fn clone_repo(repo_url: &str, local_path: &Path) -> Result<(), Error> {
    let mut callbacks = RemoteCallbacks::new();
    let username_env = dotenv!("BITBUCKET_USER");
    let password_env = dotenv!("BITBUCKET_PASSWORD");
    callbacks.credentials(|_url, _username, _allowed| {
        // Provide your credentials here
        Cred::userpass_plaintext(username_env, password_env)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_options);

    repo_builder.clone(repo_url, local_path)?;

    Ok(())
}

pub async fn create_new_branch(repo_path: &Path, branch_name: &str) -> Result<(), Error> {
    // Open the existing Git repository
    let repo = Repository::open(repo_path)?;

    // Get the HEAD reference (current branch or commit)
    let head = repo.head()?;

    // Resolve the HEAD reference to an AnnotatedCommit
    let annotated_commit = repo.reference_to_annotated_commit(&head)?;

    // Resolve the AnnotatedCommit to a Commit
    let commit = repo.find_commit(annotated_commit.id())?;

    // Create a new branch from the Commit
    repo.branch(branch_name, &commit, false)?;

    Ok(())
}

pub async fn checkout_branch(repo_path: &Path, branch_name: &str) -> Result<(), git2::Error> {
    // Open the Git repository
    let repo = Repository::open(repo_path)?;

    // Lookup the branch by name
    let branch = repo.find_branch(branch_name, BranchType::Local)?;

    // Get the target commit of the branch
    let target_commit = branch.into_reference().peel_to_commit()?;

    // Convert the target commit to an &Object
    let target_object: &Object = target_commit.as_object();

    // Update the HEAD to point to the branch
    repo.set_head(&format!("refs/heads/{}", branch_name))?;

    // Checkout the branch (optional)
    let mut checkout_builder = CheckoutBuilder::new();
    repo.checkout_tree(target_object, Some(&mut checkout_builder))?;

    Ok(())
}

pub async fn commit_changes(
    repo_path: &Path,
    author_name: &str,
    author_email: &str,
    commit_message: &str,
) -> Result<(), git2::Error> {
    // Open the existing Git repository
    let repo = Repository::open(repo_path)?;

    // Get the index (staging area) to stage changes
    let mut index = repo.index()?;

    // Stage changes you want to commit (e.g., modifying files)
    // For example, you can use repo.index.add_path("file.txt") to stage a file.
    // Make sure to add all the changes you want to include in the commit.

    // Committer's identity
    let signature = Signature::now(author_name, author_email)?;

    // Commit the changes
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Get the current HEAD commit (or None if it's the initial commit)
    let parent_commit = match repo.head().and_then(|r| r.peel_to_commit()) {
        Ok(commit) => Some(commit),
        _ => None,
    };

    if let Some(parent) = parent_commit {
        repo.commit(
            Some("HEAD"), // Point HEAD to the new commit
            &signature,
            &signature,
            commit_message,
            &tree,
            &[&parent],
        )?;
    } else {
        repo.commit(
            Some("HEAD"), // Point HEAD to the new commit
            &signature,
            &signature,
            commit_message,
            &tree,
            &[], // Empty array because there's no parent commit
        )?;
    }

    Ok(())
}

pub async fn push_to_repository(repo_path: &Path, branch_name: &str) -> Result<(), String> {
    // Open the Git repository
    let repo = match git2::Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to open repository: {}", e)),
    };

    // Get the remote (e.g., "origin")
    let mut remote = match repo.find_remote("origin") {
        Ok(remote) => remote,
        Err(e) => return Err(format!("Failed to find remote 'origin': {}", e)),
    };

    // Set up remote callbacks for authentication
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username, _allowed| {
        // Provide your credentials here
        Cred::userpass_plaintext("nvanamen", "ATBBpPMd3yzBChW3P8W3Mf8J5AsCADDF67AA")
    });

    // Set up push options with remote callbacks
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // Push the branch to the remote
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    match remote.push(&[&refspec], Some(&mut push_options)) {
        Ok(_) => {
            println!("Pushed branch '{}'", branch_name);
            Ok(())
        }
        Err(e) => Err(format!("Failed to push branch '{}': {}", branch_name, e)),
    }
}
pub async fn git_add_file(repo_path: &Path, file_path: &str) -> Result<(), git2::Error> {
    // Open the Git repository
    let repo = Repository::open(repo_path)?;

    // Get the repository's index
    let mut index = repo.index()?;

    // Add the file to the index
    index.add_path(file_path.as_ref())?;

    // Write the index to update the staging area
    index.write()?;

    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct PullRequest {
    // Define the fields you need for your pull request
    // For example:
    pub title: String,
    pub description: String,
    pub source_branch: String,
    pub destination_branch: String,
    pub base_url: String,
    pub project_key: String,
    pub repository_slug: String,
    // Add other fields as needed
}

pub(crate) async fn create_pull_request(pull_request: PullRequest) -> Result<(), reqwest::Error> {
    let base_url = pull_request.base_url;
    let project_key = pull_request.project_key;
    let repository_slug = pull_request.repository_slug;
    let url = format!(
        "https://api.{}/2.0/repositories/{}/{}/pullrequests",
        base_url, project_key, repository_slug
    );

    let client = Client::new();

    let payload = json!({
        "title": pull_request.title,
        "description": pull_request.description,
        "toRef": {
            "id": format!("refs/heads/{}", pull_request.destination_branch),
            "type": "BRANCH",
            // Add other fields as needed
        },
        "fromRef": {
            "id": format!("refs/heads/{}", pull_request.source_branch),
            "type": "BRANCH",
            // Add other fields as needed
        },
    });

    let response = client
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer <access_token>")
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Pull request created successfully.");
    } else {
        eprintln!(
            "Failed to create pull request. Status code: {:?}",
            response.status()
        );
    }

    Ok(())
}
