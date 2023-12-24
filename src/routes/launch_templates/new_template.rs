use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use hcl::{Block, Body};

use crate::routes::launch_templates::LaunchTemplate;

pub async fn new_launch_template(template: LaunchTemplate, local_path: &Path) -> Result<(), Error> {
    let device_tags = template.device_tags;
    let name_tag = device_tags[0].to_string();
    let owner_tag = device_tags[1].to_string();
    let cicd_tag = device_tags[2].to_string();

    let body = Body::builder()
        .add_block(
            Block::builder("resource")
                .add_label(template.aws_launch_template)
                .add_label(template.name.to_string())
                .add_attribute(("disable_api_termination", template.disable_api_termination))
                .add_attribute(("image_id", template.image_id))
                .add_attribute(("instance_type", template.instance_type))
                .add_attribute(("key_name", template.key_name))
                .add_attribute(("name", template.name.clone()))
                .add_attribute(("tags", ""))
                .add_attribute(("tags_all", ""))
                .add_block(
                    Block::builder("iam_instance_profile")
                        .add_attribute(("arn", template.iam_instance_profile_arn))
                        .build(),
                )
                .add_block(
                    Block::builder("network_interfaces")
                        .add_attribute(("device_index", "0"))
                        .add_attribute(("ipv4_address_count", "0"))
                        .add_attribute(("ipv4_addresses", "[]"))
                        .add_attribute(("ipv4_prefix_count", "0"))
                        .add_attribute(("ipv4_prefixes", "[]"))
                        .add_attribute(("ipv6_address_count", "0"))
                        .add_attribute(("ipv6_addresses", "[]"))
                        .add_attribute(("ipv6_prefix_count", "0"))
                        .add_attribute(("ipv6_prefixes", "[]"))
                        .add_attribute(("network_card_index", "0"))
                        .add_attribute(("security_groups", template.security_groups))
                        .add_attribute(("subnet_id", template.subnet_id))
                        .build(),
                )
                .add_block(
                    Block::builder("tag_specifications")
                        .add_attribute(("resource_type", "instance"))
                        .add_block(
                            Block::builder("tags")
                                .add_attribute(("Name", name_tag))
                                .add_attribute(("Owner", owner_tag))
                                .add_attribute(("CICD", cicd_tag))


                                .build(),
                        )
                        .build(),
                )
                .add_block(
                    Block::builder("lifecycle")
                        .add_attribute(("ignore_changes", "default_version"))
                        .build(),
                )

                .build(),
        )
        .build();

    let serialized = hcl::to_string(&body).unwrap();
    let local_path = format!("{}", local_path.display());
    // Specify the file output path
    let file_path = format!("{}/{}.tf", local_path, &template.name);
    dbg!(&file_path);

    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(serialized.as_bytes())
        .expect("Failed to write to the file");



    println!("HCL code has been written to {:?}.", &file);


    Ok(())



}
