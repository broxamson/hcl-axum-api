use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use hcl::{Block, Body};
use crate::routes::load_balancer::LoadBalancer;


pub async fn new_load_balancer(lb_template: LoadBalancer, local_path: &Path) -> Result<(), Error> {


    let body = Body::builder()
        .add_block(
            Block::builder("resource")
                .add_label("aws_lb")
                .add_label(lb_template.name.to_string())
                .add_attribute(("enable_cross_zone_load_balancing", true))
                .add_attribute(("enable_deletion_protection", false))
                .add_attribute(("internal", lb_template.internal))
                .add_attribute(("ip_address_type", "ipv4"))
                .add_attribute(("load_balancer_type", lb_template.lb_type.to_string()))
                .add_attribute(("name", lb_template.name.to_string()))
                .add_attribute(("preserve_host_header", false))
                .add_attribute(("security_groups", lb_template.security_groups))
                .add_attribute(("subnets", lb_template.subnet_id))
                .add_attribute(("tags", "".to_string()))
                .add_attribute(("tags_all", "".to_string()))
                .build(),
        )
        .add_block(
            Block::builder("resource")
                .add_label("aws_lb_target_group")
                .add_label(lb_template.name.to_string())
                .add_attribute(("name", lb_template.name.to_string()))
                .add_attribute(("port", lb_template.port.to_string()))
                .add_attribute(("protocol", lb_template.protocol.to_string()))
                .add_attribute(("target_type", lb_template.target_type.to_string()))
                .add_attribute(("vpc_id", lb_template.vpc_id.to_string()))
                .add_block(
                    Block::builder("health_check")
                        .add_attribute(("enabled", true))
                        .add_attribute(("interval", 30))
                        .add_attribute(("port", "traffic-port"))
                        .add_attribute(("protocol", lb_template.protocol.to_string()))
                        .build(),
                )
                .build(),
        )
        .build();




    let serialized = hcl::to_string(&body).unwrap();
    let local_path = format!("{}", local_path.display());
    // Specify the file output path
    let file_path = format!("{}/{}.tf", local_path, &lb_template.name);
    dbg!(&file_path);

    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(serialized.as_bytes())
        .expect("Failed to write to the file");



    println!("HCL code has been written to {:?}.", &file);


    Ok(())



}
