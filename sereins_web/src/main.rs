use std::env;
use axum::{Router, Server};
use log::info;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono::Local;
use sereis_web::apps::admin::controller::routers;
use sereis_web::{APPLICATION_CONTEXT, init_context};
use sereis_web::initialize::config::{ApplicationConfig};

#[tokio::main]
async fn main() {

    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:jia111111@localhost:3306/house");

    let mut sql = format!("insert into rent_device (sn,mac,val,state,type,house_id,time_put) values");
    let time = Local::now().naive_local().to_string();

    for i in 0..1000 {
        sql.push_str(&format!("('sn','mac',1.2,1,'电',{},'{}'),", i, time.clone()))
    }
    let sql = sql.trim_end_matches(',');

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let time = std::time::Instant::now();
    let res = sqlx::query(sql)
        .execute(&pool)
        .await
        .unwrap();

    println!("执行耗时{:?},插入数据{:?}", time.elapsed(), res.rows_affected());


    // // 初始化项目
    // init_context().await;
    //
    // // 项目路由
    // let app = Router::new()
    //     .nest("/admin", routers())
    //     .fallback(|| async { "not found" });
    //
    // let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    // let addr = config.server_config.service_format();
    //
    // // 启动服务
    // info!("http服务启动:{}",config.server_config.http_addr());
    // Server::bind(&addr.parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap()
}