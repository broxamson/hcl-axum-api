extern crate rusoto_core;
extern crate rusoto_ec2;

use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::routes::ec2::lib::launch_instance_from_template;

mod lib;

#[derive(Serialize, Deserialize)]
pub struct LaunchTemplateParams {
    pub template_name: String,
}

pub async fn launch_ec2(
    template: Query<LaunchTemplateParams>,
) -> Result<Json<String>, axum::http::StatusCode> {
    let template_name = template.template_name.to_string();
    match launch_instance_from_template(&template_name).await {
        Ok(result) => {
            let result_str = format!("{:?}", result); // Convert to a string representation
            Ok(Json(result_str))
        }
        Err(e) => {
            eprintln!("Error Launching Template: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
