use tokio;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;
use std::sync::{ mpsc, Arc, Mutex};
use colored::Colorize;
use rfd::FileDialog;
use mysql_async::{prelude::*, Conn, Opts, Pool};
use console::{self, Term};

mod utils;
use crate::utils::connection::connection;
use crate::utils::print_text::print_text;
use utils::print_text::Colors;

#[tokio::main]
#[warn(unused_assignments)]
async fn main() {
	print_text("Qual o nome do banco de dados?", Colors::Green);
	let mut db_name = String::new();
	io::stdin().read_line(&mut db_name).expect("Erro ao gravar nome do db");

	let (mut connection, mut conn, db_host) = connection().await;

	let conn_query = conn.query_drop(format!("create database {}", &db_name)).await;
  match conn_query {
    Ok(_) => {
      connection = Pool::new(Opts::from_url(&format!("{}/{}", &db_host, &db_name)).unwrap());
      conn = connection.get_conn().await.unwrap();
    },
    Err(_) => {
			print_text("Esse Banco existe, deseja sobreescrever? S/N", Colors::Green);
			let mut confirm: String = String::new();
			io::stdin().read_line(&mut confirm).expect("Erro ao receber os dados");
			let confirm = confirm.trim().to_lowercase().chars().next().unwrap();
			match confirm {
				's' => {
					conn.query_drop(format!("drop database {}", &db_name)).await.unwrap();
					conn.query_drop(format!("create database {}", &db_name)).await.unwrap();
					connection = Pool::new(Opts::from_url(&format!("{}/{}", &db_host, &db_name)).unwrap());
					conn = connection.get_conn().await.unwrap();
				},
				'n' => exit(1),
				_ => exit(1)
			};

    }
  }

	print_text("Deseja selecionar o arquivo? S/N", Colors::Green);
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap();
	let choice = choice.trim().to_lowercase().chars().next().unwrap();
	match choice {
		's' => execute_task(connection, conn).await,
		'n' => println!("{}", "Até mais 👋".blue().bold()),
		_ => println!("{}", "Somente S/N".red().bold()),
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
		None => println!("{}", "Não escolheu nenhum arquivo".red().bold())
	}
}

async fn install_file(file: PathBuf, connection: Pool, mut conn: Conn)  {
	let file_read = File::open(&file).expect("Erro ao ler o arquivo");
	let reader = BufReader::new(&file_read);

	let mut command_block = String::new();
	let start_time = Instant::now();

	println!("{}", "Em andamento, aguarde...".green().bold());

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

	println!("{}", "Sucesso ao inserir os dados 🤙".green().bold());
	println!("{}", format!("Tempo decorrido da instalação: {:?}", elapsed_time).purple().bold());

	println!("{}", "Digite qualquer tecla para sair...".white().bold());
	let mut touch_exit = String::new();
	io::stdin().read_line(&mut touch_exit).unwrap();
	if !touch_exit.is_empty() {
		exit(1);
	}
}