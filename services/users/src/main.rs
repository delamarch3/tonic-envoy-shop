use tonic::transport::Server;
use users::db::database_connection;
use users::rpc::{Service, UsersServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = database_connection().await?;
    let service = Service::new(pool);

    Server::builder()
        .accept_http1(true)
        .add_service(UsersServer::new(service))
        .serve("0.0.0.0:50000".parse()?)
        .await?;

    Ok(())
}
