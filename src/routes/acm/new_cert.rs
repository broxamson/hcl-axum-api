use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use hcl::{Block, Body};
use crate::routes::acm::CertInput;


pub async fn new_cert(cert_input: CertInput, local_path: &Path) -> Result<(), Error> {
    let cert_tf = cert_input.domain_name.clone();
    let tags = if cert_input.tags.is_some(){
        cert_input.tags
    } else {
        None
    };

    let tags_all = if cert_input.tags_all.is_some(){
        cert_input.tags_all
    } else {
        None
    };

    //formats the string to add TF data 
    let body = Body::builder()
        .add_block(
            Block::builder("resource")
                .add_label("aws_acm_certificate")
                .add_label(&cert_input.domain_name)
                .add_attribute(("domain_name", cert_input.domain_name))
                .add_attribute(("subject_alternative_names", cert_input.subject_alternative_names))
                .add_attribute(("validation_method", cert_input.validation_method.to_string()))
                .add_attribute(("tags", tags.is_some()))
                .add_attribute(("tags_all", tags_all.is_some()))
                .build(),
        )
        .add_block(
            Block::builder("options")
                .add_attribute(("certificate_transparency_logging_preference", "ENABLED"))
                .build(),
        )
        .build();


    let serialized = hcl::to_string(&body).unwrap();
    let local_path = format!("{}", local_path.display());
    // Specify the file output path
    let file_path = format!("{}/{}.tf", local_path, cert_tf);
    dbg!(&file_path);

    // Create or open the file for writing
    let mut file = File::create(&file_path).expect("Failed to create the file");

    // Write the generated HCL to the file
    file.write_all(serialized.as_bytes())
        .expect("Failed to write to the file");



    println!("HCL code has been written to {:?}.", &file);


    Ok(())



}
