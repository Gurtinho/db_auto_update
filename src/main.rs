use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;
use rfd::FileDialog;
use mysql_async::{prelude::*, Pool, Conn};
use tokio;

mod colors;
use colors::*;

async fn connection() -> (Pool, Conn) {
  let connect = Pool::new("mysql://root:sa1234@localhost:3306/icomp934_teste");
  println!("Conexão estabelecida com sucesso");
  let conn = connect.get_conn().await.unwrap();
	(connect, conn)
}

#[tokio::main]
async fn main() {
	print_text_colored("Deseja selecionar o arquivo? S/N", Colors::Yellow);

	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap();
	let choice = choice.trim().to_lowercase().chars().next().unwrap();

	match choice {
		's' => {
			let files = FileDialog::new()
				.add_filter("sql", &["sql"])
				.set_directory("/")
				.pick_file();

			match files {
				Some(file) => {
					install_file(file).await
				},
				None => print_text_colored("Não escolheu nenhum arquivo", Colors::Red)
			}
		},
		'n' => print_text_colored("Até mais 👋", Colors::Purple),
		_ => print_text_colored("Somente S/N", Colors::Red),
	}
}


async fn install_file(file: PathBuf)  {
	let file_read = File::open(&file).expect("Erro ao ler o arquivo");
	let reader = BufReader::new(&file_read);

	let (connection, mut conn) = connection().await;

	let start_time = Instant::now();

	let mut command_block = String::new();

	for line in reader.lines() {
		// o let vai capturar o valor do Result<String, Error>
		if let Ok(sql_line) = line {
			// se a linha for diferente de vazio e não for comentário
			if !&sql_line.trim().starts_with('-') && !&sql_line.trim().is_empty() {
				if sql_line.trim().ends_with(';') {
					command_block.push_str(&sql_line);
					let command = format!(r"{}", &command_block);
					conn.query_drop(&command).await.expect("Erro ao rodar script sql");
					command_block.clear();
				} else {
					command_block.push_str(&sql_line);
				}
			}
		}
	}

	if !command_block.trim().is_empty() {
		let command = format!(r"{}", &command_block);
		conn.query_drop(&command).await.expect("Erro ao rodar script sql");
		command_block.clear();
	}

	drop(conn);
	connection.disconnect().await.unwrap();

	let elapsed_time = Instant::now() - start_time;

	print_text_colored("Sucesso ao inserir os dados 🤙", Colors::Green);
	print_text_colored(&format!("Tempo decorrido da instalação: {:?}", elapsed_time), Colors::Purple);
}