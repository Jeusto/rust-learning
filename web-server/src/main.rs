use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/index.html"),
        "GET /favicon.ico HTTP/1.1" => ("HTTP/1.1 200 OK", "public/favicon.ico"),
        _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    };

    let response: String;
    let file_content = fs::read(filename);

    match &file_content {
        Ok(file_content) => {
            response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n",
                status_line,
                file_content.len(),
            );
        }
        Err(_) => {
            let status_line = "HTTP/1.1 505 INTERNAL SERVER ERROR";
            response = format!("{}\r\n\r\n", status_line);
        }
    }

    stream.write_all(response.as_bytes()).unwrap();
    stream.write(&file_content.unwrap()).unwrap();
    stream.flush().unwrap();
}
