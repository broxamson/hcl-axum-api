use axum::{Router, routing::post};
use crate::routes::acm::cert_api;
use crate::routes::asg::asg_api;

use crate::routes::ec2::launch_ec2;
use crate::routes::launch_templates::lt_api;
use crate::routes::load_balancer::lb_api;
use crate::routes::s3::bucket_api;

mod ec2;
mod s3;
mod launch_templates;
mod load_balancer;
mod acm;
mod asg;

pub async fn create_routes() -> Router {
    Router::new()


        .route("/api/s3/create_bucket", post(bucket_api))
        .route("/api/ec2/launch_instance", post(launch_ec2))
        .route("/api/ec2/create_lt", post(lt_api))
        .route("/api/ec2/create_lb", post(lb_api))
        .route("/api/acm/create_cert", post(cert_api))
        .route("/api/sg/create_asg", post(asg_api))
}
