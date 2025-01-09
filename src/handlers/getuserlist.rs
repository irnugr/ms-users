use actix_web::{get, web, Responder, HttpResponse};
use tokio_postgres::NoTls;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct UserList {
    pub userid: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
}

#[derive(Serialize)]
pub struct PaginatedResponse {
    pub total_items: i64,
    pub users: Vec<UserList>,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub offset: i64,
    pub limit: i64,
}

#[get("/users/get-user-list")]
pub async fn get_user_list(pagination: web::Query<PaginationParams>) -> impl Responder {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=postgres password=postgres123456", NoTls)
            .await
            .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let offset = pagination.offset;
    let limit = pagination.limit;

    // Query to get the total number of users
    let total_items: i64 = client
        .query_one("SELECT COUNT(*) FROM public.user", &[])
        .await
        .expect("Failed to execute count query")
        .get(0);

    // Query to get the paginated user list
    let rows = client
        .query(
            "SELECT user_id, username, email, is_active FROM public.user ORDER BY user_id ASC LIMIT $1 OFFSET $2",
            &[&limit, &offset]
        )
        .await
        .expect("Failed to execute query");

    let mut user_list: Vec<UserList> = Vec::new();

    for row in rows {
        let user_item = UserList {
            userid: row.get(0),
            username: row.get(1),
            email: row.get(2),
            is_active: row.get(3),
        };
        user_list.push(user_item);
    }

    let response = PaginatedResponse {
        total_items,
        users: user_list,
    };

    HttpResponse::Ok().json(response)
}