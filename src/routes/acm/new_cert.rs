use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use hcl::{Block, Body};


pub async fn new_cert(lb_template: LoadBalancer, local_path: &Path) -> Result<(), Error> {
    //formats the string to add TF data 





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
