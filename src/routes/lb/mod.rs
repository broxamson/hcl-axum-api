use std::path::Path;

use anyhow::Result;
use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::routes::git_func::{checkout_directory, clone_repo, commit_changes, create_new_branch, create_pull_request, git_add_file, PullRequest, push_to_repository};
use crate::routes::lb::new_lb::new_load_balancer;

mod new_lb;

const REPO_URL: &str = "https://bitbucket/netreo/terraform";


#[derive(Serialize, Deserialize)]
pub struct LoadBalancer {
    r#type: String,
    name: String,
    subnets: Vec<String>,
    security_groups: Vec<String>,
    scheme: String,
    load_balancer_attributes: Vec<LoadBalancerAttribute>,
}

#[derive(Serialize, Deserialize)]
pub struct LoadBalancerAttribute {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Listener {
    r#type: String,
    default_actions: Vec<DefaultAction>,
    load_balancer_arn: LoadBalancerArn,
    port: i32,
    protocol: String,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultAction {
    r#type: String,
    fixed_response_config: FixedResponseConfig,
}

#[derive(Serialize, Deserialize)]
pub struct FixedResponseConfig {
    content_type: String,
    status_code: String,
    content_body: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoadBalancerArn {
    r#ref: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoadBalancerQuery {
    load_balancer: LoadBalancer,
    load_balancer_attr: LoadBalancerAttribute,
    listener: Listener,
    default_action: DefaultAction,
    fixed_response: FixedResponseConfig,
    load_balancer_arn: LoadBalancerArn,
}

pub async fn lb_api(
    Query(load_balancer_param): Query<LoadBalancerQuery>,
) -> Result<Json<String>, axum::http::StatusCode> {
    let branch_name = load_balancer_param.load_balancer.name.to_string();
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

    if let Err(err) = new_load_balancer(load_balancer_param).await {
        eprintln!("Error: {:?}", err);
    }
    println!("Branch {} Checked Out.", branch_name);


// ADDS TF FILE TO GIT

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
