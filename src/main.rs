extern crate byteorder;
extern crate bytes;
extern crate rgckms;

use std::slice::Iter;
use std::net::TcpListener;
use std::thread;
use std::time;
use std::net::TcpStream;

use byteorder::{WriteBytesExt};
use std::io::{Write, Read};
use std::sync::Mutex;
use std::sync::Arc;
use std::net::UdpSocket;
use rgckms::Server;
use rgckms::ServerList;


fn main() {
    println!("GCK Master Server");
    let server_list = Arc::new(Mutex::new( ServerList::new()));

    let server = Server::new("136.143.97.184".to_string(), "19711".to_string());
    let server2 = Server::new("artolsheim.hipstercat.fr".to_string(), "19711".to_string());

    {
        let mut list = server_list.lock().unwrap();

        list.add(server);
        list.add(server2);
    }

    println!("\nServer List: {:?}", server_list);

    // Listen for the hearth beat.
    let heart_beat_socket = UdpSocket::bind("0.0.0.0:27900")
        .expect("Could not bind hearth beat socket to port 27900");

    let listener = TcpListener::bind("0.0.0.0:28900")
        .expect("Could not bind query socket to port 28900");

    // TODO Receive from the hearth beat socket.

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let server_list_c = server_list.clone();

        let _hearth_beat = thread::spawn(move || {
            println!("Connection established!");
            handle_query_connection(stream, server_list_c);
        });
    }
}

const SERVER_SEPARATOR: u8 = 0xAC;

fn handle_query_connection(mut stream: TcpStream, server_list_mutex: Arc<Mutex<ServerList>>) {
    // "0¬163.158.182.243¬19711\0"

    let mut wtr = vec![];

    {
        let server_list = server_list_mutex.lock().unwrap();

        server_list.servers()
            .take(99)
            .enumerate()
            .for_each(|(index, server)| {
                let index_format = format!("{:02}", index);
                let index_bytes = index_format.as_bytes();
                let ip_bytes = server.ip.as_bytes();
                let port_bytes = server.port.as_bytes();

                if index != 0 {
                    wtr.write_u8(SERVER_SEPARATOR).unwrap();
                }

                stream.write_all(index_bytes).unwrap();
                stream.write_u8(SERVER_SEPARATOR).unwrap();
                stream.write_all(ip_bytes).unwrap();
                stream.write_u8(SERVER_SEPARATOR).unwrap();
                stream.write_all(port_bytes).unwrap();
                stream.write_u8(0).unwrap();
            });

        stream.write_u8(0).unwrap();
        stream.flush().unwrap();
    }

    // Hack for now to keep the connection open.
    let mut buf = [0; 128];

    loop {
        let i = stream.read(&mut buf).unwrap();

        if i == 0 {
            break;
        }

        println!("Read timeout: {:02X?}", &buf[..i]);

        let sleep_time = time::Duration::from_millis(1000);
        thread::sleep(sleep_time);
    }
}
