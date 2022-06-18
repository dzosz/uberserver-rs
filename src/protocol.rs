use log::{debug, error, info};
use std::net::SocketAddr;
use std::net::UdpSocket;

use crate::client::Client;

#[derive(Default)]
pub struct Protocol {}

pub trait Command {
    fn get_function_args(&mut self, args: &str) -> Result<(), String>;
    fn execute(&self, client: &mut Client);
}

#[derive(Default)]
struct PingCommand {
    response: Option<String>,
}

impl Command for PingCommand {
    fn get_function_args(&mut self, args: &str) -> Result<(), String> {
        let arg_num = 1;

        let mut parts = args.splitn(arg_num, ' ').fuse();

        self.response = match parts.next() {
            Some("") | None => None,
            Some(v) => Some(v.to_string()),
        };
        Ok(())
    }

    fn execute(&self, client: &mut Client) {
        match self.response {
            Some(ref v) => client.Send(&format!("PONG {}", v)),
            None => client.Send("PONG {}"),
        }
    }
}

struct SayCommand {
}

impl SayCommand {
}

#[derive(Default)]
struct PortTestCommand {
    host : String,
    port : usize,
}

impl Command for PortTestCommand  {
    fn get_function_args(&mut self, args: &str) -> Result<(), String> {
        let arg_num = 2;

        let mut parts = args.splitn(arg_num, ' ').fuse();

        self.host = parts.next()
            .ok_or_else(|| "Missing host argument")?
            .into();
        self.port = parts.next()
            .ok_or_else(|| "Missing port argument")?
            .parse()
            .map_err(|_| "Can't parse port argument")?; 
        Ok(())
    }

    fn execute(&self, client: &mut Client) {
        debug!("Executing PortTestCommand {}:{}", self.host, self.port);
        let local = "0.0.0.0:0".parse::<SocketAddr>().unwrap();
        match UdpSocket::bind(&local) {
            Ok(socket) => {
                let target = format!("{}:{}", self.host, self.port);
                socket.send_to(b"Port testing...", target);
            },
            Err(_) => {
                error!("Could not open udp socket on {}:{} in PortTestCommand", self.host, self.port);
            }
        }
    }
}

impl Protocol {
    fn get_function(command: &str) -> Option<Box<dyn Command>> {
        match command {
            "PING" => Some(Box::new(PingCommand::default())),
            "PORTTEST" => Some(Box::new(PortTestCommand::default())),
            _ => None
        }
    }

    pub fn _handle(&self, msg: &str) -> Result<Box<dyn Command>, String> {
        let mut fun = None;

        // TODO implement restricted list handling
        //if command not in self.restricted_list:

        let command = {
            if let Some((command, args)) = msg.split_once(' ') {
                let command = &command.to_lowercase();
                // TODO add error checking for max cargs size
                fun = Protocol::get_function(command);
                if let Some(ref mut v) = fun {
                    v.get_function_args(args)?;
                }
                command.to_string()
            } else {
                let command = &msg.to_lowercase();
                fun = Protocol::get_function(&command);
                command.to_string()
            }
        };

        match fun {
            None => Err(format!(
                "{} failed. Unknown command. (args='{}')",
                command, msg
            )),
            Some(v) => {
                Ok(v)
            }
        }
    }
}
