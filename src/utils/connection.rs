use mysql_async::{prelude::*, Opts, Pool, Conn};

pub async fn connection(db_name: &String) -> (Pool, Conn) {
  let db_host = "mysql://root:sa1234@localhost:3306";
  let connect = Pool::new(Opts::from_url(&db_host).unwrap());
  let mut conn = connect.get_conn().await.unwrap();
  let conn_query = conn.query_drop(format!("create database {}", &db_name)).await;
  match conn_query {
    Ok(_) => {
      let connect = Pool::new(Opts::from_url(&format!("{}/{}", &db_host, &db_name)).unwrap());
      let conn = connect.get_conn().await.unwrap();
      (connect, conn)
    },
    Err(_) => {
      conn.query_drop(format!("drop database {}", &db_name)).await.unwrap();
			conn.query_drop(format!("create database {}", &db_name)).await.unwrap();
			let connect = Pool::new(Opts::from_url(&format!("{}/{}", &db_host, &db_name)).unwrap());
			let conn = connect.get_conn().await.unwrap();
			(connect, conn)
    }
  }
}
