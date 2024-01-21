use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;
use rfd::FileDialog;
use mysql_async::{prelude::*, Pool, Conn};
use tokio;

mod utils;
use crate::utils::connection::connection;

fn print_text(text: &str) {
	print!("{} ", text.to_string());
	io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
	print_text("Qual o nome do banco de dados?");
	let mut db_name = String::new();
	io::stdin().read_line(&mut db_name).expect("Erro ao gravar nome do db");

	let (connection, conn) = connection(&db_name).await;

	print_text("Deseja selecionar o arquivo? S/N");
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap();
	let choice = choice.trim().to_lowercase().chars().next().unwrap();
	match choice {
		's' => execute_task(connection, conn).await,
		'n' => println!("Até mais 👋"),
		_ => println!("Somente S/N"),
	}
}

async fn execute_task(connection: Pool, conn: Conn) {
	let files = FileDialog::new()
		.add_filter("sql", &["sql"])
		.set_directory("/")
		.pick_file();
	match files {
		Some(file) => {
			install_file(file, connection, conn).await
		},
		None => println!("Não escolheu nenhum arquivo")
	}
}

async fn install_file(file: PathBuf, connection: Pool, mut conn: Conn)  {
	let file_read = File::open(&file).expect("Erro ao ler o arquivo");
	let reader = BufReader::new(&file_read);

	let mut command_block = String::new();
	let start_time = Instant::now();

	println!("Em andamento, aguarde...");
	for line in reader.lines() {
		if let Ok(sql_line) = line {
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

	println!("Sucesso ao inserir os dados 🤙");
	println!("Tempo decorrido da instalação: {:?}", elapsed_time);
	let mut touch_exit = String::new();
	io::stdin().read_line(&mut touch_exit).unwrap();
	if !touch_exit.is_empty() {
		exit(1);
	}
}