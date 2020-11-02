pub mod docker {
    use std::path::Path;

    use futures_util::stream::TryStreamExt;
    use hyper::Client;
    use hyperlocal::{UnixClientExt, Uri};

    pub async fn get_containers() -> Result<String, Box<dyn std::error::Error>> {
        let path = Path::new("/var/run/docker.sock");
        let url = Uri::new(path, "/containers/json").into();
        let client = Client::unix();
        let response_body = client.get(url).await?.into_body();
        let bytes = response_body
            .try_fold(Vec::default(), |mut buf, bytes| async {
                buf.extend(bytes);
                Ok(buf)
            })
            .await?;
        let result = String::from_utf8(bytes)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use testcontainers::{clients, images, Docker};
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_containers() {
        let generic_postgres = images::generic::GenericImage::new("library/postgres:13")
            .with_wait_for(images::generic::WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", "trend")
            .with_env_var("POSTGRES_USER", "rusty")
            .with_env_var("POSTGRES_PASSWORD", "rusty");
        let docker = clients::Cli::default();
        let container = docker.run(generic_postgres);
        let container_id = &container.id()[..12];
        Runtime::new().unwrap().block_on(async {
            let result = super::docker::get_containers().await;
            assert_eq!(format!("{}", container_id), result.unwrap());
        });
    }
}
