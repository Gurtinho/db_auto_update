use mysql_async::{Opts, Pool, Conn};

pub async fn connection() -> (Pool, Conn, String) {
  let db_host = "mysql://root:sa1234@localhost:3306";
  let connect = Pool::new(Opts::from_url(&db_host).unwrap());
  let conn = connect.get_conn().await.unwrap();
  (connect, conn, db_host.to_string())
}
