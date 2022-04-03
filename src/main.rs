mod get;
mod post;

use std::error::Error;
use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;




fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args.is_empty(){
        println!("Specify a port you moron: Try cargo run <port number>")
    }
    let port = format!("127.0.01:{}",args[1]);

    let connection = TcpListener::bind(port).unwrap();




    for stream in connection.incoming() {


        let mut stream = stream.unwrap();
        let mut buffer = [0;1024];
        stream.read(&mut buffer).unwrap();
        let stringed_buffer = String::from_utf8_lossy(&buffer).to_string();


        let http_request = stringed_buffer.lines().next().unwrap().to_string();
        let mut http_request_array:Vec<&str> =Vec::new();
        for word in http_request.split_whitespace(){
            http_request_array.push(word)
        }


        let http_method = http_request_array[0];
        let params = http_request_array[1];
        let http_version = http_request_array[2];


        //Response based on the http_method
        if http_method == "GET" {
            let content = get::get_method(params);
            let response_status = format!("{} 200 OK\r\n",http_version);
            let response = format!("{}Content-len: {}\r\n\r\n{}",response_status, content.len(),content);
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();


        }else if http_method == "POST" {

        }else {

        }

    }

}

