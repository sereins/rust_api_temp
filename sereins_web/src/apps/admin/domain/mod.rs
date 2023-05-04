use sqlx::{Error, MySql, Pool, QueryBuilder};
use sqlx::mysql::MySqlQueryResult;
use sqlx::types::chrono::{Local, NaiveDateTime};
use crate::apps::admin::entities::Admin;

pub async fn admin(pool: &Pool<MySql>) -> Vec<Admin> {
    let sql = "select * from rent_auth";

    let res: Vec<Admin> = sqlx::query_as(sql)
        .fetch_all(pool)
        .await
        .unwrap();
    res
}


pub async fn adds(pool: &Pool<MySql>) -> u64 {
    let mut sql = format!("insert into rent_device (sn,mac,val,state,type,house_id,time_put) values");

    let time = Local::now().naive_local().to_string();
    for i in 0..1000 {
        sql.push_str(&format!("('sn','mac',1.2,1,'电',{},'{}'),", i, time.clone()))
    }

    let sql = sql.trim_end_matches(',');

    let res = sqlx::query(sql)
        .execute(pool)
        .await;
    match res {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            println!("错误{:?}", e);
            0
        }
    }
}