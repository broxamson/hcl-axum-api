use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use hcl::Body;
use crate::routes::load_balancer::LoadBalancer;


pub async fn new_load_balancer(template: LoadBalancer, local_path: &Path) -> Result<(), Error> {
    let device_tags = template.device_tags;
    let name_tag = device_tags[0].to_string();
    let owner_tag = device_tags[1].to_string();
    let cicd_tag = device_tags[2].to_string();

    let body = Body::builder()


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
