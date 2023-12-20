use std::fs::File;
use std::io::Write;

use anyhow::{Error, Result};

use crate::routes::launch_templates::LaunchTemplate;

pub async fn new_template(launch_template: LaunchTemplate) -> Result<(), Error> {
    let json_str = serde_json::to_string(&launch_template)?;

    // Deserialize JSON string to HCL Body
    let hcl_body = hcl::to_string(&json_str)?;

    // Use the deserialized HCL Body as needed
    dbg!(&hcl_body);
    let file_path = format!("tf/{}/modules/dev_lt/{}.tf", &launch_template.name, &launch_template.name);
    dbg!(&file_path);
    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(hcl_body.as_bytes()).expect("Failed to write to the file");


    Ok(())
}






