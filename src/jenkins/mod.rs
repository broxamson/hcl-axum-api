use anyhow::Error;
use dotenvy_macro::dotenv;

pub async fn trigger_tf_check(
    branch: &str,
    aws_type: &str,
    env: &str,
    region: &str,
) -> Result<String, Error> {
    let auth_token = dotenv!("AUTH_TOKEN");
    let auth_string = format!("Basic {}", auth_token);
    let client = reqwest::Client::builder().build()?;
    let jenkins_host = dotenv!("JENKINS_HOST");

    let auth_header_value = reqwest::header::HeaderValue::from_str(
        &auth_string,
    )?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, auth_header_value);
    headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"));

    let json_data = serde_json::json!({
        "terraform_branch": branch,
        "terraform_command": "plan -no-color",
        "terraform_service": aws_type,
    });

    let request = client
        .post(format!(
            "{}/job//Terraform/job/{}/job/{}/job/TERRAFORM-PLAN-APPLY/buildWithParameters",
            jenkins_host, env, region
        ))
        .headers(headers)
        .json(&json_data);

    let response = request.send().await?;

    // Check the response status
    if response.status().is_success() {
        let body = response.text().await?;
        println!(
            "Jenkins job triggered successfully! Response body: {}",
            &body
        );

        // Return the response body as a String
        Ok(body)
    } else {
        let status_code = response.status();
        println!(
            "Failed to trigger Jenkins job. Status code: {}",
            status_code
        );

        // Return an error with the status code
        Err(anyhow::anyhow!("Failed to trigger Jenkins job. Status code: {}", status_code))
    }
}
