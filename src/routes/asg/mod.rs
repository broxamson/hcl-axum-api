
use std::path::Path;

use anyhow::Result;
use axum::Json;
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};

use crate::git_func::{checkout_branch, clone_repo, commit_changes, create_new_branch, create_pull_request, delete_comitted_change, git_add_file, PullRequest, push_to_repository};
use crate::routes::asg::new_asg::new_asg;


mod new_asg;

const REPO_URL: &str = "https://bitbucket.org/netreo/terraform";
const REPO_PATH: &str = dotenv!("REPO_DIR");


#[derive(Serialize, Deserialize, Clone)]
pub struct ASGInput {

    count                     : u8,
    capacity_rebalance        : bool,

    default_instance_warmup   : u8,
    desired_capacity          : u8,
    enabled_metrics           : Vec<String>,
    health_check_grace_period : u16,
    health_check_type         : String,
    load_balancers            : Vec<String>,
    max_instance_lifetime     : u8,
    max_size                  : u8,
    metrics_granularity       : String,
    min_size                  : u8,
    name                      : String,
    protect_from_scale_in     : bool,
    service_linked_role_arn   : String,

    target_group_arns         : Vec<String>,
    termination_policies : Vec<String>,
    vpc_zone_identifier  : Vec<String>,
    lt_id : String,
    lt_version: String,
    lc_ignore_changes: Vec<String>,


}


pub async fn asg_api(
    Json(asg_input): Json<ASGInput>,
) -> Result<Json<String>, axum::http::StatusCode> {
    let branch_name = asg_input.name.clone();

    let asg_input_json = ASGInput {
        count: asg_input.count,
        capacity_rebalance: asg_input.capacity_rebalance,

        default_instance_warmup: asg_input.default_instance_warmup,
        desired_capacity: asg_input.desired_capacity,
        enabled_metrics: asg_input.enabled_metrics,
        health_check_grace_period: asg_input.health_check_grace_period,
        health_check_type: asg_input.health_check_type,
        load_balancers: asg_input.load_balancers,
        max_instance_lifetime: asg_input.max_instance_lifetime,
        max_size: asg_input.max_size,
        metrics_granularity: asg_input.metrics_granularity,
        min_size: asg_input.min_size,
        name: asg_input.name,
        protect_from_scale_in: asg_input.protect_from_scale_in,
        service_linked_role_arn: asg_input.service_linked_role_arn,

        target_group_arns: asg_input.target_group_arns,
        termination_policies: asg_input.termination_policies,
        vpc_zone_identifier: vec![],
        lt_id: asg_input.lt_id,
        lt_version: asg_input.lt_version,
        lc_ignore_changes: asg_input.lc_ignore_changes,
    };



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
    let file_path  = Path::new(&branch_dir).join("modules/dev_asg_input");


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

    if let Err(err) = new_asg(asg_input_json, &file_path).await {
        eprintln!("Error: {:?}", err);
    }
    println!("File written to {} .", branch_name);


// WRITES THE TF FILE

    let file_name = format!("modules/dev_asg_input/{}.tf",  &branch_name );

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
