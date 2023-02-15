use sereis_web::init_context;

#[tokio::main]
async fn main() {

    // 初始化项目
    init_context().await


    // let app = Router::new()
    //     .route("/admin", get(index_handel));
    //
    //
    // let socket = SocketAddr::from(([127,0,0,1],8000));
    //
    // // 启动服务
    // Server::bind(&socket)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap()
}

