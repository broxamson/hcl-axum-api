#![recursion_limit = "512"]

use std::fs::File;
use std::io::Write;
use anyhow::{Result, Error};
use askama::filters::json;
use axum::Json;
use hcl::{Block, Body, body};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};


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




pub async fn new_template(launch_template: LaunchTemplate)  {
    let launch_template_json = LaunchTemplate {
        aws_launch_template: "aws_launch_template".to_string(),
        default_version: 1,
        disable_api_termination: false,
        image_id: "ami-0c2b8ca1dad447f8a".to_string(),
        instance_type: "t2.micro".to_string(),
        key_name: "rhel8gold".to_string(),
        name: "launch_template".to_string(),
        iam_instance_profile_arn: "arn:aws:iam::123456789012:instance-profile/ecsInstanceRole".to_string(),
        security_groups: vec!["default".to_string()],
        subnet_id: "subnet-1234567890abcdef0".to_string(),
        device_tags: "device_tags".to_string(),
    };
    let json_string = serde_json::to_string(&launch_template_json).expect("Failed to convert to JSON string");

    let value: Value = hcl::from_str(json_string).unwrap();

    }






