use std::path::Path;

use anyhow::Result;
use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::routes::git_func::{checkout_directory, clone_repo, commit_changes, create_new_branch, create_pull_request, git_add_file, PullRequest, push_to_repository};
use crate::routes::launch_templates::new_template::new_template;

mod new_template;

const REPO_URL: &str = "https://bitbucket/netreo/terraform";

#[derive(Serialize, Deserialize)]
pub struct LaunchTemplate {
    aws_launch_template: String,
    default_version: u8,
    disable_api_termination: bool,
    image_id: String,
    instance_type: String,
    key_name: String,
    name: String,
    iam_instance_profile_arn: String,
    security_groups: Vec<String>,
    subnet_id: String,
    device_tags: String,

}



pub async fn lt_api(
    Query(launch_template): Query<LaunchTemplate>,
) -> Result<Json<String>, axum::http::StatusCode> {

    let launch_template_json = LaunchTemplate {
        aws_launch_template: launch_template.aws_launch_template.to_string(),
        default_version: launch_template.default_version,
        disable_api_termination: launch_template.disable_api_termination,
        image_id: launch_template.image_id.to_string(),
        instance_type: launch_template.instance_type.to_string(),
        key_name: launch_template.key_name.to_string(),
        name: launch_template.name.to_string(),
        iam_instance_profile_arn: launch_template.iam_instance_profile_arn.to_string(),
        security_groups: launch_template.security_groups,
        subnet_id: launch_template.subnet_id.to_string(),
        device_tags: launch_template.device_tags.to_string(),
    };



    let branch_name = launch_template.name.to_string();
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

    if let Err(e) = checkout_directory(local_path, &branch_name, local_path).await {
        eprintln!("Error Checking out Branch: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Branch {} Checked Out.", branch_name);


// CREATES THE .TF FILE

    if let Err(err) = new_template(launch_template_json).await {
        eprintln!("Error: {:?}", err);
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
