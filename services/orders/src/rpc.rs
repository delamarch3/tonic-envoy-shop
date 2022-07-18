mod proto {
    tonic::include_proto!("orders");
}

use crate::user_client::{get_user_credit_limit, UsersClient};
use proto::orders_server::Orders;
pub use proto::orders_server::OrdersServer;
use proto::place_order_response::Status as PlaceOrderStatus;
pub use proto::Order;
use proto::{
    GetUserOrdersRequest, GetUserOrdersResponse, PlaceOrderRequest,
    PlaceOrderResponse,
};
use sqlx::PgPool;
use tonic::transport::Channel;
use tonic::{Code, Request, Response, Status};

pub struct Service {
    pool: PgPool,
    users: UsersClient<Channel>,
}

impl Service {
    pub fn new(pool: PgPool, users: UsersClient<Channel>) -> Self {
        Self { pool, users }
    }
}

#[tonic::async_trait]
impl Orders for Service {
    async fn get_user_orders(
        &self,
        request: Request<GetUserOrdersRequest>,
    ) -> Result<Response<GetUserOrdersResponse>, Status> {
        let request = request.into_inner();
        match Order::get_by_userid(self.pool.clone(), request.userid).await {
            Ok(orders) => {
                let reply = GetUserOrdersResponse { orders };
                Ok(Response::new(reply))
            }
            Err(e) => {
                eprintln!("{}", e);
                let status = Status::new(Code::Internal, e.to_string());
                Err(status)
            }
        }
    }
    async fn place_order(
        &self,
        request: Request<PlaceOrderRequest>,
    ) -> Result<Response<PlaceOrderResponse>, Status> {
        let request = request.into_inner();
        let mut users = self.users.clone();
        let credit_limit = get_user_credit_limit(&mut users, request.userid)
            .await
            .unwrap();

        let user_total =
            Order::add_user_total(self.pool.clone(), request.userid)
                .await
                .unwrap();

        let remaining = credit_limit as f32 - user_total - request.total;

        if remaining < 0. {
            let status =
                Status::new(Code::FailedPrecondition, "Insufficient credit");
            return Err(status);
        }

        match Order::create(
            self.pool.clone(),
            request.userid,
            request.product,
            request.total,
        )
        .await
        {
            Ok(_) => {
                let reply = PlaceOrderResponse {
                    status: PlaceOrderStatus::Success as i32,
                };
                Ok(Response::new(reply))
            }
            Err(e) => {
                eprintln!("{}", e);
                let status = Status::new(Code::Internal, e.to_string());
                Err(status)
            }
        }
    }
}
