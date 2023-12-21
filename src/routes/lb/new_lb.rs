use std::fs::File;
use std::io::Write;

use anyhow::{Error, Result};

use crate::routes::lb::{DefaultAction, FixedResponseConfig, Listener, LoadBalancer, LoadBalancerArn, LoadBalancerAttribute, LoadBalancerQuery};

pub async fn new_load_balancer(load_balancer_param: LoadBalancerQuery) -> Result<(), Error> {
    let load_balancer = LoadBalancer {
        r#type: load_balancer_param.load_balancer.r#type.to_string(),
        name: load_balancer_param.load_balancer.name.to_string(),
        subnets: load_balancer_param.load_balancer.subnets,
        security_groups: load_balancer_param.load_balancer.security_groups,
        scheme: load_balancer_param.load_balancer.scheme,
        load_balancer_attributes: vec![LoadBalancerAttribute {
            key: load_balancer_param.load_balancer_attr.key.to_string(),
            value: load_balancer_param.load_balancer_attr.value.to_string(),
        }],
    };

    let lb_listener = Listener {
        r#type: load_balancer_param.listener.r#type,
        default_actions: vec![DefaultAction {
            r#type: load_balancer_param.default_action.r#type,
            fixed_response_config: FixedResponseConfig {
                content_type: load_balancer_param.fixed_response.content_type.to_string(),
                status_code: load_balancer_param.fixed_response.status_code,
                content_body: load_balancer_param.fixed_response.content_body,
            },
        }],
        load_balancer_arn: LoadBalancerArn { r#ref: load_balancer_param.load_balancer_arn.r#ref.to_string() },
        port: load_balancer_param.listener.port,
        protocol: load_balancer_param.listener.protocol,
    };


    let json_lb = serde_json::to_string(&load_balancer)?;
    let json_list = serde_json::to_string(&lb_listener)?;

    // Deserialize JSON string to HCL Body
    let hcl_lb = hcl::to_string(&json_lb)?;
    let hcl_list = hcl::to_string(&json_list)?;

    // Use the deserialized HCL Body as needed
    dbg!(&hcl_lb, &hcl_list);
    let file_path = format!("tf/{}/modules/dev_lt/{}.tf", &load_balancer_param.load_balancer.name, &load_balancer_param.load_balancer.name);
    dbg!(&file_path);
    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(hcl_lb.as_bytes()).expect("Failed to write LB to the file");
    file.write_all(hcl_list.as_bytes()).expect("Failed to write Listener to the file");


    Ok(())
}






