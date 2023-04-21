use ssh2::{Agent, Channel, Session};
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let username: &str = "username";
    let password: &str = "your_password";
    let tcp = TcpStream::connect("localhost:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(username, password).unwrap();
    println!("{}", sess.authenticated());

    let mut channel: Channel = sess.channel_session().unwrap();
    channel.exec("ls -la").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
}
