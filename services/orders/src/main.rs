use orders::db::database_connection;
use orders::rpc::{OrdersServer, Service};
use orders::user_client::users_connection;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = database_connection().await?;
    let users_client = users_connection().await?;
    let service = Service::new(pool, users_client);

    Server::builder()
        .accept_http1(true)
        .add_service(OrdersServer::new(service))
        .serve("0.0.0.0:50001".parse()?)
        .await?;

    Ok(())
}
