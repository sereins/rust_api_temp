// use axum::extract::{
//     Path, WebSocketUpgrade,
//     ws::{Message, WebSocket},
// };
// use futures::{stream::StreamExt};
// use axum::response::{IntoResponse, Response};
// use lazy_static::lazy_static;
// use dashmap::{DashMap};
// use futures_channel::mpsc::{unbounded, UnboundedSender};
// use log::warn;
// use crate::apps::resp::{CODE_NOT_LOGIN, RespVo};
// use crate::common::{CODE_NOT_LOGIN, RespVo};
// use crate::middleware::user_token::UserToken;
//
// // 全局变量 用户id和stock发送端集合
// lazy_static! {
//     pub static ref USERS:DashMap<String,UnboundedSender<Message>> = DashMap::new() ;
// }
//
// /// socket
// pub async fn handler(ws: WebSocketUpgrade, Path(token): Path<String>) -> Response {
//     // 获取用户信息
//     let user = user_info_by_token(token).await;
//
//     match user {
//         Some(user) => {
//             ws.on_upgrade(|socket| { handle_socket(socket, user) })
//         }
//         None => {
//             RespVo::waring_with_code(CODE_NOT_LOGIN, "未登录", &"".to_owned())
//                 .resp_json().into_response()
//         }
//     }
// }
//
// /// 处理stock连接
// async fn handle_socket(socket: WebSocket, user: UserToken) {
//     let (user_tx, mut user_rx) = socket.split();
//
//     // 创建一个channel
//     let (tx, rx) = unbounded();
//     // 保存为全局变量方便调用(k:用户id , v:发送端)
//     USERS.insert(user.org_admin.clone(), tx);
//
//     // 发送任务
//     let mut send_task = tokio::spawn(rx.map(Ok).forward(user_tx));
//
//     // 读取任务
//     let user1 = user.clone();
//     let mut recv_task = tokio::spawn(async move {
//         while let Some(result) = user_rx.next().await {
//             match result {
//                 Ok(msg) => {
//                     handle_msg(msg, user1.clone()).await;
//                 }
//                 Err(e) => {
//                     println!("e = {}", e);
//                 }
//             }
//         }
//     });
//
//     tokio::select! {
//         _v1 = &mut recv_task => send_task.abort(),
//         _v2 = &mut send_task => recv_task.abort(),
//     }
//
//     warn!("connect closed");
//     // 清除存在用户
//     USERS.remove(&*user.org_admin.clone()).unwrap();
// }
//
// /// 处理消息
// async fn handle_msg(msg: Message, user: UserToken) {
//     // 获取到发送端
//     let sender = USERS.get(&user.org_admin).unwrap();
//     match msg {
//         // 当前直接返回消息
//         Message::Text(msg) => {
//             sender.unbounded_send(Message::Text(msg)).expect("TODO: panic message");
//         }
//         // 接收到ping后返回pong
//         Message::Ping(_) => {
//             sender.unbounded_send(Message::Pong(vec![1])).expect("TODO: panic message");
//         }
//         // 连接关闭移除用户
//         Message::Close(_) => {
//             USERS.remove(&*user.org_admin.clone()).unwrap();
//         }
//         _ => {}
//     }
// }