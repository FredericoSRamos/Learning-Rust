use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::process::exit;

fn main() {
    println!("\n\n[Cliente] Processo cliente começou a rodar.\n");

    let mut nome = String::new();
    let porta: u16;

    println!("[Cliente] Quer usar o arquivo de configuração? (s/n):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "s" {
        let file = File::open("./server.data").expect("[Cliente] Erro ao abrir o arquivo de configuração");
        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        nome = lines.next().unwrap().unwrap();
        porta = lines.next().unwrap().unwrap().parse().unwrap();
    } else {
        println!("\n[Cliente] Qual o nome do servidor?");
        std::io::stdin().read_line(&mut nome).unwrap();
        nome = nome.trim().to_string();

        input.clear();

        println!("\n[Cliente] Qual a porta do serviço?");
        std::io::stdin().read_line(&mut input).unwrap();
        porta = input.trim().parse().unwrap();

        let file = File::create("./server.data").expect("\nErro ao criar arquivo de configuração");
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}\n{}", nome, porta).unwrap();
    }

    println!("\n[Cliente] Usando o servidor {} e a porta {}.\n", nome, porta);

    let mut stream = match TcpStream::connect(format!("{}:{}", nome, porta)) {
        Ok(stream_valid) => stream_valid,
        Err(e) => {
            println!("[Cliente] Erro no connect: {}", e);
            exit(0);
        }
    };

    let msg = "Pode me prestar um serviço?";
    stream.write_all(msg.as_bytes()).expect("[Cliente] Erro no write.");

    let mut buffer = [0; 80];
    let bytes_read = stream.read(&mut buffer).expect("[Cliente] Erro no read.");
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    
    println!("[Cliente] Recebeu: {}", response);
}