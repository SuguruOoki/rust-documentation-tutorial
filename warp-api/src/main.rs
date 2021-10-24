use warp_api::{init_db, users_api};

#[tokio::main]
async fn main() {
	// Database(HashMap)の初期化
	let database = init_db();

	// users_api filterにdatabaseを代入してサーバを起動
	warp::serve(users_api(database))
		.run(([127, 0, 0, 1], 3030))
		.await;
}
