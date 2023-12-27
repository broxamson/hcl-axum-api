use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use hcl::{Block, Body};

use crate::routes::asg::ASGInput;


pub async fn new_asg(cert_input: ASGInput, local_path: &Path) -> Result<(), Error> {
    let tf_file_name = cert_input.name.clone();
    let attribute_list =
        "force_delete,
        force_delete_warm_pool
        wait_for_capacity_timeout";

    //formats the string to add TF data 
    let body = Body::builder()
        .add_block(
            Block::builder("resource")
                .add_label("aws_autoscaling_group")
                .add_label(&cert_input.name)
                .add_attribute(("count", cert_input.count))
                .add_attribute(("capacity_rebalance", cert_input.capacity_rebalance))
                .add_attribute(("default_cooldown", 0))
                .add_attribute(("default_instance_warmup", cert_input.default_instance_warmup))
                .add_attribute(("desired_capacity", cert_input.desired_capacity))
                .add_attribute(("enabled_metrics", cert_input.enabled_metrics))
                .add_attribute(("health_check_grace_period", cert_input.health_check_grace_period))
                .add_attribute(("health_check_type", cert_input.health_check_type))
                .add_attribute(("load_balancers", cert_input.load_balancers))
                .add_attribute(("max_instance_lifetime", cert_input.max_instance_lifetime))
                .add_attribute(("max_size", cert_input.max_size))
                .add_attribute(("metrics_granularity", cert_input.metrics_granularity))
                .add_attribute(("min_size", cert_input.min_size.to_string()))
                .add_attribute(("name", cert_input.name))
                .add_attribute(("protect_from_scale_in", cert_input.protect_from_scale_in))
                .add_attribute(("service_linked_role_arn", cert_input.service_linked_role_arn))
                .add_attribute(("suspended_processes", "[]"))
                .add_attribute(("target_group_arns", cert_input.target_group_arns))
                .add_attribute(("termination_policies", cert_input.termination_policies))
                .add_attribute(("vpc_zone_identifier", cert_input.vpc_zone_identifier))


                .build(),
        )
        .add_block(
            Block::builder("launch_template")
                .add_attribute(("id ", cert_input.lt_id))
                .add_attribute(("version ", "$Latest"))
                .build(),
        )
        .add_block(
            Block::builder("lifecycle")
                .add_attribute(("ignore_changes ", attribute_list))

                .build(),
        ).add_block(
        Block::builder("timeouts")


            .build()
    )
        .build();


    let serialized = hcl::to_string(&body).unwrap();
    let local_path = format!("{}", local_path.display());
    // Specify the file output path
    let file_path = format!("{}/{}.tf", local_path, tf_file_name);
    dbg!(&file_path);

    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(serialized.as_bytes())
        .expect("Failed to write to the file");



    println!("HCL code has been written to {:?}.", &file);


    Ok(())



}
