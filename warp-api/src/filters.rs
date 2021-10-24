use warp::{Filter, Rejection, Reply};

use crate::{get_user_handler, list_users_handler, put_user_handler, Database};

/// 最終的に公開するFilter
/// 用意した部品を組み合わせて表現する
pub fn users_api(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	get_user(db.clone()).or(list(db.clone())).or(put_user(db))
}

/// Path "users" を表す部品
fn users() -> warp::filters::BoxedFilter<()> {
	warp::path("users").boxed()
}

/// PathからUserIdを取り出す部品
fn user_id() -> warp::filters::BoxedFilter<(u64,)> {
	warp::path::param().boxed()
}

/// list_users_handlerを呼び出すための部品
fn list(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	users().and(warp::get()) // HTTP GETメソッドを指定
		.and_then(move || list_users_handler(db.clone())) // Handlerを呼び出す
}

/// get_user_handlerを呼び出すための部品
fn get_user(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	users().and(user_id()) // User IdをPathから取得
		.and(warp::get()) // HTTP GETメソッドを指定
		.and_then(move |id| get_user_handler(db.clone(), id)) // Handlerを呼び出す
}

/// put_user_handlerを呼び出すための部品
fn put_user(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	users().and(user_id()) // User IdをPathから取得
		.and(warp::put()) // HTTP PUTメソッドを指定
		.and(warp::body::json()) // Request Bodyに含まれたJSONを取り出しUser型へ変換
		.and_then(move |id, body| put_user_handler(db.clone(), id, body)) // Handlerを呼び出す
}
