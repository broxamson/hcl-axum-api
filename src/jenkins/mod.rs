use std::str::FromStr;

pub async fn _trigger_tf_check(
    branch: &str,
    tf_type: &str,
    env: &str,
    region: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let jenkins_host = "http://172.16.1.47:8080";
    let auth_header_value = reqwest::header::HeaderValue::from_str(
        "Basic bnZhbmFtZW46MTE5OGM1NWRkOTU2YTIxNjAxMmI3MDA1OTM2NTc4N2M2NA==",
    )?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, auth_header_value);

    let json_data = serde_json::json!({
        "terraform_branch": branch,
        "terraform_command": "plan -no-color",
        "terraform_service": tf_type,
    });

    let request = client
        .post(format!(
            "/job/{}/Terraform/job/{}/job/{}/job/TERRAFORM-PLAN-APPLY/buildWithParameters",
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
            body
        );
    } else {
        println!(
            "Failed to trigger Jenkins job. Status code: {}",
            response.status()
        );
    }

    Ok(())
}
