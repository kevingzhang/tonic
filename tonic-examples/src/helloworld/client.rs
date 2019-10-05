pub mod docker_api {
    tonic::include_proto!("docker_api");
}

use docker_api::{client::GetDockerClient, DockerRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GetDockerClient::connect("http://[::1]:50051")?;

    let request = tonic::Request::new(DockerRequest {
        name: "hello".into(),
    });

    let response = client.get_docker_info(request).await?;

    println!("RESPONSE={:?}", response);
    let request = tonic::Request::new(DockerRequest {
        name: "hello".into(),
    });
    let response = client.get_docker_images(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}
