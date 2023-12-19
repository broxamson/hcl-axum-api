use std::fs::File;
use hcl::{Block, Body};

pub async fn new_template() -> Result<(), _> {
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
        );

    let file_path = format!("tf/{}/modules/dev_s3/{}.tf", &bucketname, &bucket_name);
    dbg!(&file_path);
    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(serialized.as_bytes())
        .expect("Failed to write to the file");

    crate::routes::s3::new_bucket::clean_file(&file_path).expect("Failed to Clean to the file");

    println!("HCL code has been written to {:?}.", &file);



    Ok(())

}