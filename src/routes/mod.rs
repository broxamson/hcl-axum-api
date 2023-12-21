use axum::{Router, routing::post};

use crate::routes::ec2::launch_ec2;
use crate::routes::launch_templates::lt_api;
use crate::routes::s3::bucket_api;

mod ec2;
mod git_func;
mod s3;
mod launch_templates;
mod lb;

pub async fn create_routes() -> Router {
    Router::new()


        .route("/api/s3/create_bucket", post(bucket_api))
        .route("/api/ec2/launch_instance", post(launch_ec2))
        .route("/api/ec2/create_lt", post(lt_api))
}
