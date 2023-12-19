use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use axum::Json;
use hcl::{Block, Body};



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
    device_tags: Json<String>,

}




pub async fn new_template(launch_template: LaunchTemplate) -> Result<(), Err()> {
    let security_groups = launch_template.security_groups.iter().to_owned().collect();

    let body = Body::builder()
        .add_block(
            Block::builder("resource")
                .add_label("aws_launch_template")
                .add_label(launch_template.aws_launch_template)
                .add_attribute(("default_version", launch_template.default_version))
                .add_attribute(("disable_api_termination", launch_template.disable_api_termination))
                .add_attribute(("image_id", launch_template.image_id))
                .add_attribute(("instance_type", launch_template.instance_type))
                .add_attribute(("key_name", launch_template.key_name))
                .add_attribute(("name", launch_template.name))
                .add_attribute(("tags", {}))
                .add_attribute(("tags_all", {}))
                .add_block(
                    Block::builder("iam_instance_profile")
                        .add_attribute(("arn", launch_template.iam_instance_profile_arn))
                        .build(),
                )
                .add_block(
                    Block::builder("network_interfaces")
                        .add_attribute(("device_index", 0))
                        .add_attribute(("ipv4_address_count", 0))
                        .add_attribute(("ipv4_addresses", []))
                        .add_attribute(("ipv4_prefix_count", 0))
                        .add_attribute(("ipv4_prefixes", []))
                        .add_attribute(("ipv6_address_count", 0))
                        .add_attribute(("ipv6_addresses", []))
                        .add_attribute(("ipv6_prefix_count", 0))
                        .add_attribute(("ipv6_prefixes", []))
                        .add_attribute(("network_card_index", 0))
                        .add_attribute(("security_groups", security_groups))
                        .add_attribute(("subnet_id", launch_template.subnet_id))
                        .build(),
                )
                .add_block(
                    Block::builder("tag_specifications")
                        .add_attribute(("resource_type", "instance"))
                        .add_attribute(("tags", launch_template.device_tags))
                        .build(),
                )
                .add_block(
                    Block::builder("lifecycle")
                        .add_attribute(("ignore_changes", ["default_version"]))
                        .build(),
                ).build());



    let serialized = hcl::to_string(&body).unwrap();
    let file_path = format!("tf/{}/modules/dev_lt/{}.tf", &launch_template.aws_launch_template, &launch_template.aws_launch_template);
    dbg!(&file_path);
    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(serialized.as_bytes())
        .expect("Failed to write to the file");



    println!("HCL code has been written to {:?}.", &file);



    Ok(())

}