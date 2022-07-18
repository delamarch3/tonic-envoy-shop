mod proto {
    tonic::include_proto!("users");
}

use crate::model::User;
use proto::users_server::Users;
pub use proto::users_server::UsersServer;
use proto::{
    GetUserCreditLimitRequest, GetUserCreditLimitResponse, GetUserRequest,
    GetUserResponse,
};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub struct Service {
    pool: PgPool,
}

impl Service {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Users for Service {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let request = request.into_inner();

        let user = User::get(self.pool.clone(), request.id).await.unwrap();
        let reply = GetUserResponse {
            id: user.id,
            firstname: user.firstname,
            lastname: user.lastname,
            credit_limit: user.credit_limit,
        };

        Ok(Response::new(reply))
    }

    async fn get_user_credit_limit(
        &self,
        request: Request<GetUserCreditLimitRequest>,
    ) -> Result<Response<GetUserCreditLimitResponse>, Status> {
        let request = request.into_inner();

        let credit_limit =
            User::get_credit_limit(self.pool.clone(), request.id)
                .await
                .unwrap();

        let reply = GetUserCreditLimitResponse { credit_limit };

        Ok(Response::new(reply))
    }
}
