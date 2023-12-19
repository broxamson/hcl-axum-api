mod new_template;



use crate::routes::git_func::{
    checkout_branch, clone_repo, commit_changes, create_new_branch, create_pull_request,
    git_add_file, push_to_repository, PullRequest,
};

use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::Path;

const REPO_URL: String = "https://bitbucket/netreo/terraform";

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    bucket_name: String,
}

pub async fn bucket_api(
    Query(bucket_name): Query<QueryParams>,
) -> Result<Json<String>, axum::http::StatusCode> {
    let branch_name = bucket_name.bucket_name.to_string(); // Replace with your branch name
    let pull_request = PullRequest {
        title: branch_name.to_string(),
        description: format!("Creating new Bucket: {}", branch_name).to_string(),
        source_branch: branch_name.to_string(),
        destination_branch: "master".to_string(),
        base_url: "bitbucket.org".to_string(),
        project_key: "netreo".to_string(),
        repository_slug: "terraform".to_string(),
    };

    let url_base = pull_request.base_url.to_string();
    // The URL of the Git repository you want to clone
    let repo_url = REPO_URL;

    let branch_dir = format!("tf/{}", branch_name);
    let local_path = Path::new(&branch_dir);

// CLONES THE REPO

    if let Err(e) = clone_repo(&repo_url, local_path).await {
        eprintln!("Error cloning repository: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Repository cloned successfully to {:?}", local_path);


// CREATES NEW BRANCH

    if let Err(e) = create_new_branch(local_path, &branch_name).await {
        eprintln!("Error branching repository: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Branch {} Created.", branch_name);


// CHECKS OUT SAID BRANCH

    if let Err(e) = checkout_branch(local_path, &branch_name).await {
        eprintln!("Error Checking out Branch: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Branch {} Checked Out.", branch_name);


// CREATES THE .TF FILE

    if let Err(e) = new_template().await {
        eprintln!("Error Creating Launch Template TF: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Branch {} Checked Out.", branch_name);


// WRITES THE TF FILE

    let file_name = format!("modules/dev_lt/{}.tf", branch_name);

    if let Err(e) = git_add_file(local_path, &file_name).await {
        eprintln!("Error adding file to the staging area: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("File added to the staging area.");


// COMMITS TO GIT

        if let Err(e) =
        commit_changes(local_path, &branch_name, "nicholas", "nvanamen@netreo.com").await
    {
        eprintln!("Error committing and pushing changes: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Changes committed and pushed successfully.");

    // PUSH TO GIT

    if let Err(e) = push_to_repository(local_path, &branch_name).await {
        eprintln!("Error pushing to the remote repository: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!(
        "Branch '{}' pushed successfully to the remote repository.",
        &branch_name
    );

    // CREATE PR

    if let Err(err) = create_pull_request(pull_request).await {
        eprintln!("Error: {:?}", err);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json("Success".to_string()))
}
