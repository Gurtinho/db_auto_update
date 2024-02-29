use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use colored::Colorize;
use rfd::FileDialog;
use mysql_async::{prelude::*, Conn, Pool};
use console::{self, Term};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{ mpsc, Arc, Mutex};
use std::io::{self, BufRead, BufReader};

pub async fn execute_task(connection: Pool, conn: Conn) {
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


	// thread pra imprimir os segundos passando...
	// move faz com que a thread receba dados do ambiente externo
	// preciso ajustar pra que quando acabe a execução do db, interrompa a contagem
  // thread::spawn(move || {
	// 	let term = Term::stdout();
	// 	let mut seconds = 0;
	// 	loop {
	// 		seconds = seconds + 1;
	// 		term.write_line(&format!("Tempo decorrido: {:?}", seconds).to_string().purple().bold()).unwrap();
	// 		thread::sleep(Duration::from_millis(1000));
	// 		term.clear_last_lines(1).unwrap();
	// 	}
  // }).join().unwrap();
	

	// let t_runtime = tokio::runtime::Runtime::new().unwrap();
	// t_runtime.block_on(async move {
	// 	tokio::spawn(async move {

	// 	});
	// });


	for line in reader.lines() {
		if let Ok(sql_line) = line {
			if !&sql_line.trim().starts_with('-') && !&sql_line.trim().is_empty() {
				if sql_line.trim().ends_with(';') {
					command_block.push_str(&sql_line);
					let command = format!(r"{}", &command_block);
					conn.query_drop(&command).await.expect("Erro ao rodar script sql");
					// println!("{}", command);
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