extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{
    DescribeLaunchTemplatesRequest, Ec2, Ec2Client, LaunchTemplateSpecification,
    RunInstancesRequest,
};

pub async fn _list_launch_templates() -> Result<(), Box<dyn std::error::Error>> {
    // Specify your AWS region here
    let region = Region::default();

    // Create an AWS EC2 client
    let client = Ec2Client::new(region);

    // Create a request to list launch templates
    let request = DescribeLaunchTemplatesRequest {
        ..Default::default()
    };

    // Send the request to AWS and await the response
    let result = client.describe_launch_templates(request).await?;

    // Loop through the launch templates and print their names
    for template in result.launch_templates.unwrap_or_default() {
        let template_name = template.launch_template_name.unwrap_or_default();
        println!("{}", &template_name);
    }

    Ok(())
}

pub async fn launch_instance_from_template(
    template: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Specify your AWS region
    let region = Region::default();

    // Create an EC2 client
    let client = Ec2Client::new(region);

    // Specify the launch template name
    let launch_template_name = Some(template.to_string());

    // Specify other parameters as needed
    let instance_type = "m6a.large";
    let key_name = "rhel8gold";
    let lt_version = None; // Use the default version of the launch template

    // Create a request to launch an instance from the launch template
    let request = RunInstancesRequest {
        launch_template: Some(LaunchTemplateSpecification {
            launch_template_id: None,
            launch_template_name,
            version: lt_version,
        }),
        min_count: 1,
        max_count: 1,
        instance_type: Some(instance_type.to_string()),
        key_name: Some(key_name.to_string()),
        ..Default::default()
    };

    // Send the request to launch an instance
    let response = client.run_instances(request).await;

    match response {
        Ok(result) => {
            if let Some(instance) = result.instances {
                if let Some(instance_id) = &instance[0].instance_id {
                    println!("Instance ID: {}", instance_id);

                    Ok(instance_id.to_string()) // Return Ok when successful
                } else {
                    Err("No instance was launched.".to_string().into()) // Return a generic error
                }
            } else {
                Err("No instance was launched.".to_string().into()) // Return a generic error
            }
        }
        Err(err) => {
            eprintln!("Error launching instance: {:?}", err);
            Err(err.into()) // Return the error as-is
        }
    }
}
