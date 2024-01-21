use mysql_async::{Pool, Conn};
use super::colors::{print_text_colored, Colors};

pub async fn connection() -> (Pool, Conn) {
  let connect = Pool::new("mysql://root:sa1234@localhost:3306/icomp934_teste");
  print_text_colored("Conexão estabelecida com sucesso", Colors::Blue);
  let conn = connect.get_conn().await.unwrap();
	(connect, conn)
}