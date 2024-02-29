use serde::de::IntoDeserializer;
use tokio;
use std::io::{self, BufRead, BufReader, Bytes};
use std::ops::Index;
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{ mpsc, Arc, Mutex};
use colored::Colorize;
use mysql_async::{prelude::*, Column, Conn, Opts, Pool, Value};
use console::{self, Term};
use serde::*;
use dialoguer::{theme::ColorfulTheme, Select};

mod utils;
mod database;
mod functions;

use utils::print_text::Colors;
use crate::utils::print_text::print_text;
use crate::functions::functions::execute_task;
use crate::database::connection::{self, Connection};

struct Row {
	values: Vec<Option<Value>>,
	columns: Arc<[Column]>,
}

#[tokio::main]
#[warn(unused_assignments)]
async fn main() {
	print_text("Qual o host? (localhost)", Colors::Yellow);
	let mut db_host = String::new();
	io::stdin().read_line(&mut db_host).expect("Erro ao receber o host");
	db_host = db_host.trim().to_string();
	if db_host.is_empty() {
		db_host = "localhost".to_string();
	}
	print_text("Qual a porta? (3306)", Colors::Yellow);
	let mut db_port = String::new();
	io::stdin().read_line(&mut db_port).expect("Erro ao receber o host");
	db_port = db_port.trim().to_string();
	if db_port.is_empty() {
		db_port = "3306".to_string();
	}
	print_text("Qual o usuário? (root)", Colors::Yellow);
	let mut db_user = String::new();
	io::stdin().read_line(&mut db_user).expect("Erro ao receber o host");
	db_user = db_user.trim().to_string();
	if db_user.is_empty() {
		db_user = "root".to_string();
	}
	print_text("Qual a senha? (root)", Colors::Yellow);
	let mut db_pass: String = String::new();
	io::stdin().read_line(&mut db_pass).expect("Erro ao receber o host");
	db_pass = db_pass.trim().to_string();
	if db_pass.is_empty() {
		db_pass = "root".to_string();
	}

	let data_connection = Connection {
		db_host,
		db_port,
		db_user,
		db_pass
	};

	let (connection, conn, db_url) = data_connection.connection().await.unwrap();

	print_text("Qual o nome do banco de dados?", Colors::Yellow);
	let mut db_name = String::new();
	io::stdin().read_line(&mut db_name).expect("Erro ao gravar nome do db");

	// PROCESSO DE SELEÇÃO DO BANCO DE DADOS

	let choice_method = vec!["backup", "upload"];

	let selected_option = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("Selecione o que deseja fazer:")
		.items(&choice_method)
		.default(0)
		.interact()
		.unwrap();

	match selected_option {
		0 => println!("Você selecionou backup"),
		1 => upload_file(conn, connection, data_connection, &db_name).await,
		_ => println!("Opção inválida"),
	}
}


async fn upload_file(mut conn: Conn, mut connection: Pool, data: Connection, db_name: &String) {
	let conn_query = conn.query_drop(format!("create database {}", &db_name)).await;

  let conn = match conn_query {
    Ok(()) => {
      connection = Pool::new(Opts::from_url(&format!("mysql://{}:{}@{}:{}/{}", &data.db_user, &data.db_pass, &data.db_host, &data.db_port, &db_name)).unwrap());
      connection.get_conn().await.unwrap()
    },
    Err(_) => {
			print_text("Esse banco de dados existe, deseja sobreescrever? S/n", Colors::Yellow);
			let mut confirm: String = String::new();
			io::stdin().read_line(&mut confirm).expect("Erro ao receber os dados");
			if confirm.is_empty() {
				confirm = "s".to_string();
			}
			let confirm: char = confirm.trim().to_lowercase().chars().next().unwrap();

			match confirm {
				's' => {
					conn.query_drop(format!("drop database {}", &db_name)).await.unwrap();
					conn.query_drop(format!("create database {}", &db_name)).await.unwrap();
					connection = Pool::new(Opts::from_url(&format!("mysql://{}:{}@{}:{}/{}", &data.db_user, &data.db_pass, &data.db_host, &data.db_port, &db_name)).unwrap());
					connection.get_conn().await.unwrap()
				},
				'n' => {
					exit(1)
				},
				_ => {
					exit(1)
				},
			}
    }
  };

	print_text("Deseja selecionar o arquivo? S/n", Colors::Yellow);
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap();
	if choice.is_empty() {
		choice = "s".to_string();
	}

	println!("{}", choice);

	let choice = choice.trim().to_lowercase().chars().next().unwrap();
	match choice {
		's' => execute_task(connection, conn).await,
		'n' => println!("{}", "Até mais 👋".blue().bold()),
		_ => return println!("{}", "Somente S/N".red().bold()),
	}
}

// async fn backup_file(mut conn: Conn, mut connection: Pool, db_url: String, db_name: String) {

// }