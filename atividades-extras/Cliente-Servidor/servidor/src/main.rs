use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::thread;
use std::time::Duration;
use gethostname::gethostname;

const PORTA: u16 = 3456;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 80];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let thread_id = thread::current().id();
            let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);

            println!("\n[Servidor - {:?}] Recebeu: {}", thread_id, received_data);

            thread::sleep(Duration::from_secs(2));

            let response = format!("Serviço prestado pela thread {:?}.", thread_id);
            stream
                .write_all(response.as_bytes())
                .expect(&(format!("\n[Servidor - {:?}] Erro no write.", thread_id))[..]);
        }
        Err(e) => {
            println!("[Servidor] Erro no read: {}", e);
        }
    }
}

fn main() {
    println!("\n\n[Servidor] Servidor entrando em funcionamento usando a porta {}.", PORTA);

    let nome : String = gethostname().into_string().unwrap();

    let listener = match TcpListener::bind(format!("{}:{}", nome, PORTA)) {
        Ok(listener_valid) => listener_valid,
        Err(e) => {
            println!("\n[Servidor] Erro no bind: {}", e);
            exit(0);
        }
    };

    println!("[Servidor] Aguardando pedido de serviço ...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match stream.peer_addr() {
                    Ok(address) => {
                        println!("\n[Servidor] Nova conexão de: {}", address)
                    }
                    Err(e) => {
                        println!("\n[Servidor] Erro ao obter endereço do cliente: {}", e);
                    }
                }
                
                thread::spawn(move || handle_client(stream));
                println!("[Servidor] Aguardando pedido de serviço ...")
            }
            Err(e) => {
                println!("\n[Servidor] Erro na conexão: {}", e);
            }
        }
    }
}