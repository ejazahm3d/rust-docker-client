use shiplift::Docker;

#[tokio::main]
async fn main() {
    let docker = Docker::new();
    match docker.containers().list(&Default::default()).await {
        Ok(containers) => {
            for c in containers {
                println!("container -> {:#?}", c);

                docker
                    .containers()
                    .get(&c.id)
                    .stop(Default::default())
                    .await
                    .expect("Error while deleting container");
            }

            let list = docker.containers().list(&Default::default()).await.unwrap();

            println!("{:#?}", list);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
