use std::path::Path;

use anyhow::Result;
use axum::Json;
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};

use crate::git_func::{checkout_branch, clone_repo, commit_changes, create_new_branch, create_pull_request, delete_comitted_change, git_add_file, PullRequest, push_to_repository};
use crate::routes::load_balancer::new_lb::new_load_balancer;

mod new_lb;

const REPO_URL: &str = "https://bitbucket.org/netreo/terraform";
const REPO_PATH: &str = dotenv!("REPO_DIR");


#[derive(Serialize, Deserialize)]
pub struct LoadBalancer {

    name: String,
    internal: bool,
    lb_type: String,
    security_groups: Vec<String>,
    subnet_id: Vec<String>,
    port: u8,
    protocol: String,
    target_type: String,
    vpc_id: String,


}


pub async fn lb_api(
    Json(load_balancer): Json<LoadBalancer>,
) -> Result<Json<String>, axum::http::StatusCode> {
    let load_balancer_json = LoadBalancer {

        name: load_balancer.name.to_string(),

        internal: load_balancer.internal,
        lb_type: load_balancer.lb_type.to_string(),
        security_groups: load_balancer.security_groups,
        subnet_id: load_balancer.subnet_id,

        port: load_balancer.port,
        protocol: load_balancer.protocol.to_string(),
        target_type: load_balancer.target_type.to_string(),
        vpc_id: load_balancer.vpc_id.to_string(),
    };


    let branch_name = load_balancer.name.to_string();
    dbg!(&branch_name);
    let pull_request = PullRequest {
        title: branch_name.to_string(),
        description: format!("Creating new Bucket: {}", branch_name).to_string(),
        source_branch: branch_name.to_string(),
        destination_branch: "master".to_string(),
        base_url: "bitbucket.org".to_string(),
        project_key: "netreo".to_string(),
        repository_slug: "terraform".to_string(),
    };

    // let url_base = pull_request.base_url.to_string();
    // The URL of the Git repository you want to clone
    let repo_url = REPO_URL;
    let branch_dir = format!("{}/tf/{}/", REPO_PATH, branch_name);
    let branch_path= Path::new(&branch_dir);
    let file_path  = Path::new(&branch_dir).join("modules/dev_load_balancer");


// CLONES THE REPO
    println!("deleting local branch {}", &branch_dir);
    delete_comitted_change(branch_dir.clone())
        .await
        .expect("Could not delete path");

    println!("{} {:?}", repo_url, branch_path);
    if let Err(e) = clone_repo(repo_url, branch_path).await {
        eprintln!("Error cloning repository: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Repository cloned successfully to {:?}", branch_path);


// CREATES NEW BRANCH


    if let Err(e) = create_new_branch(branch_path, &branch_name).await {
        eprintln!("Error branching repository: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Branch {} Created.", branch_name);


// CHECKS OUT  BRANCH

    if let Err(e) = checkout_branch(branch_path, &branch_name).await {
        eprintln!("Error Checking out Branch: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Branch {} Checked Out.", branch_name);


// CREATES THE .TF FILE

    if let Err(err) = new_load_balancer(load_balancer_json, &file_path).await {
        eprintln!("Error: {:?}", err);
    }
    println!("File written to {} .", branch_name);


// WRITES THE TF FILE

    let file_name = format!("modules/dev_load_balancer/{}.tf",  &branch_name );

    if let Err(e) = git_add_file(branch_path, &file_name).await {
        eprintln!("Error adding file to the staging area: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("File added to the staging area.");


// COMMITS TO GIT

    if let Err(e) =
        commit_changes(branch_path, &branch_name, "nicholas", "nvanamen@netreo.com").await
    {
        eprintln!("Error committing and pushing changes: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    println!("Changes committed and pushed successfully.");

    // PUSH TO GIT

    if let Err(e) = push_to_repository(branch_path, &branch_name).await {
        eprintln!("Error pushing to the remote repository: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    println!(
        "Branch '{}' pushed successfully to the remote repository.",

        &branch_name
    );

    delete_comitted_change(branch_dir.clone()).await.expect("Could not delete path");
    // CREATE PR


    if let Err(err) = create_pull_request(pull_request).await {
        eprintln!("Error: {:?}", err);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json("Success".to_string()))
}
