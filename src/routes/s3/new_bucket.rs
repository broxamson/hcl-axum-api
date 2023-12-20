use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, Write};

use hcl::{Block, Body};

pub fn new_bucket(bucket_name: &str) {
    let bucketname = bucket_name;

    let json_str = format!(
        r#"
        {{
            "Statement": [
                {{
                    "Action": "s3:GetObject",
                    "Condition": {{
                        "Bool": {{
                            "aws:SecureTransport": "false"
                        }}
                    }},
                    "Effect": "Deny",
                    "Principal": "*",
                    "Resource": "arn:aws:s3:::{}/.*"
                }}
            ],
            "Version": "2012-10-17"
        }}
        "#,
        bucket_name.trim_start()
    );

    // Pretty-print the JSON string

    let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    let pretty_json = serde_json::to_string(&json).unwrap();
    let pretty_json = &pretty_json.replace('\\', "");

    let body = Body::builder()
        .add_block(
            Block::builder("resource")
                .add_label("aws_s3_bucket")
                .add_label(bucketname.to_string())
                .add_attribute(("bucket", bucketname))
                .add_attribute(("force_destroy", "false"))
                .add_attribute(("object_lock_enabled", "false"))
                .build(),
        )
        .add_block(
            Block::builder("resource")
                .add_label("aws_s3_bucket_acl")
                .add_label(bucketname.to_string())
                .add_attribute(("bucket", format!("aws_s3_bucket.{}.id", &bucketname)))
                .add_block(
                    Block::builder("access_control_policy")
                        .add_block(
                            Block::builder("grant")
                                .add_block(
                                    Block::builder("grantee")
                                        .add_attribute((
                                            "id",
                                            "data.aws_canonical_user_id.current.id",
                                        ))
                                        .add_attribute(("type", "CanonicalUser"))
                                        .build(),
                                )
                                .add_attribute(("permission", "FULL_CONTROL"))
                                .build(),
                        )
                        .add_block(
                            Block::builder("owner")
                                .add_attribute(("id", "data.aws_canonical_user_id.current.id"))
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .add_block(
            Block::builder("resource")
                .add_label("aws_s3_bucket_lifecycle_configuration")
                .add_label(bucketname.to_string())
                .add_attribute(("bucket", format!("aws_s3_bucket.{}.id", &bucketname)))
                .add_block(
                    Block::builder("rule")
                        .add_attribute(("id", "Intelligent-Tiering"))
                        .add_block(
                            Block::builder("noncurrent_version_transition")
                                .add_attribute(("noncurrent_days", "30"))
                                .add_attribute(("storage_class", "INTELLIGENT_TIERING"))
                                .build(),
                        )
                        .add_block(
                            Block::builder("transition")
                                .add_attribute(("days", "30"))
                                .add_attribute(("storage_class", "INTELLIGENT_TIERING"))
                                .build(),
                        )
                        .add_attribute(("status", "Enabled"))
                        .build(),
                )
                .build(),
        )
        .add_block(
            Block::builder("resource")
                .add_label("aws_s3_bucket_versioning")
                .add_label(bucketname.to_string())
                .add_attribute(("bucket", format!("aws_s3_bucket.{}.id", &bucketname)))
                .add_block(
                    Block::builder("versioning_configuration")
                        .add_attribute(("status", "Enabled"))
                        .build(),
                )
                .build(),
        )
        .add_block(
            Block::builder("resource")
                .add_label("aws_s3_bucket_policy")
                .add_label(bucketname.to_string())
                .add_attribute(("bucket", format!("aws_s3_bucket.{}.bucket", &bucketname)))
                .add_attribute(("policy", format!("jsonencode({})", pretty_json)))
                .build(),
        )
        .build();

    let serialized = hcl::to_string(&body).unwrap();

    // Specify the file Out Put path
    let file_path = format!("tf/{}/modules/dev_s3/{}.tf", &bucketname, &bucket_name);
    dbg!(&file_path);
    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(serialized.as_bytes())
        .expect("Failed to write to the file");

    clean_file(&file_path).expect("Failed to Clean to the file");

    println!("HCL code has been written to {:?}.", &file);
}

pub fn clean_file(file_path: &str) -> io::Result<()> {
    // Open the file for reading and writing
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    // Read the file contents into a string
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Remove all backslashes from the content
    let modified_content = content.replace("\\", "");
    let modified_content = modified_content.replace("\"j", "j");
    let modified_content = modified_content.replace("})\"", "})");
    // Truncate the file and move the file cursor to the beginning
    file.set_len(0)?;
    file.seek(io::SeekFrom::Start(0))?;

    // Write the modified content back to the file
    file.write_all(modified_content.as_bytes())?;

    // Close the file explicitly (not required, but good practice)
    file.sync_all()?;

    println!("Backslashes removed from the file, and the file has been closed.");

    Ok(())
}
