use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use clap::{Command, Arg};
use std::env;
use std::io::{self};
use dotenv::dotenv;
use validator::validate_email;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok(); 

 
    let matches = Command::new("Kanvas CLI Email Client")  
        .version("Alpha")
        .about("Envie e-mails via SMTP pelo terminal")
        .arg(
            Arg::new("to")
                .short('t')
                .long("to")
                .takes_value(true)
                .help("Destinatário do e-mail"),
        )
        .arg(
            Arg::new("subject")
                .short('s')
                .long("subject")
                .takes_value(true)
                .help("Assunto do e-mail"),
        )
        .arg(
            Arg::new("body")
                .short('b')
                .long("body")
                .takes_value(true)
                .help("Corpo do e-mail"),
        )
        .arg(
            Arg::new("from")
                .short('f')
                .long("from")
                .takes_value(true)
                .help("Remetente do e-mail"),
        )
        .get_matches();

    fn is_valid_email(email: &str) -> bool {
        validate_email(email)
    }

    
    let to = matches.value_of("to").expect("É obrigatório ter um destinatário");
    if !is_valid_email(to) {
        eprint!("Destinatário inválido.");
        return Ok(());
    }
    
    let subject = matches.value_of("subject").unwrap_or("É necessário definir um assunto");
    let body = matches.value_of("body").unwrap_or("Conteúdo vazio");
   
    let from = matches.value_of("from").unwrap_or("deve haver um endereço válido como: seuemail@example.com");
    if !is_valid_email(from) {
        eprint!("Remetente inválido!");
        return Ok(());
    }

    
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER não encontrado, favor definir um válido");
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER não encontrado, favor definir um válido");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD não encontrado, favor definir uma válida");

    
    let email = Message::builder()
        .from(from.parse().unwrap())  
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();  

   
    let creds = Credentials::new(smtp_user, smtp_password);
    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()  
        .credentials(creds)
        .build();

    
    match mailer.send(&email) {
        Ok(_) => {
            println!("E-mail enviado com sucesso!");
        }
        Err(e) => {
            eprintln!("Falha ao enviar o e-mail, erro: {}", e);
        }
    }

    Ok(())
}
