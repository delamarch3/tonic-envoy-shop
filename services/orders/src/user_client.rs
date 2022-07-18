mod proto {
    tonic::include_proto!("users");
}

pub use proto::users_client::UsersClient;
use proto::GetUserCreditLimitRequest;
use std::env;
use tonic::transport::Channel;
use tonic::Request;

pub async fn users_connection(
) -> Result<UsersClient<Channel>, Box<dyn std::error::Error>> {
    let addr = env::var("USER_SERVICE_ADDR").unwrap();
    let client = UsersClient::connect(addr).await?;
    Ok(client)
}

pub async fn get_user_credit_limit(
    client: &mut UsersClient<Channel>,
    userid: i64,
) -> Result<i64, Box<dyn std::error::Error>> {
    let response = client
        .get_user_credit_limit(Request::new(GetUserCreditLimitRequest {
            id: userid,
        }))
        .await?;
    let response = response.into_inner();
    Ok(response.credit_limit)
}
