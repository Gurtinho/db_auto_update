use colored::Colorize;
use mysql_async::{Opts, Pool, Conn};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
  pub db_host: String,
  pub db_port: String,
  pub db_user: String,
  pub db_pass: String,
}

impl Connection {
  pub async fn connection(&self) -> Option<(Pool, Conn, String)> {
    let connect = Pool::new(Opts::from_url(&format!(
      "mysql://{}:{}@{}:{}", &self.db_user.to_string(), &self.db_pass.to_string(), &self.db_host.to_string(), &self.db_port.to_string()
    )).unwrap());
    let conn = connect.get_conn().await;
    match conn {
      Err(_) => {
        println!("{}", format!("Ocorreu um erro ao tentar se conectar").red().bold());
        None
      },
      Ok(c) => {
        println!("{}", format!("Sucesso ao conectar com o banco de dados!").green().bold());
        Some((connect, c, self.db_host.to_string()))
      }
    }
  }
}
