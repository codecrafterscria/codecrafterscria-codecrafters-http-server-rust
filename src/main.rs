// Uncomment this block to pass the first stage
use std::{
   collections::HashMap, fs, io::{Read, Write}, net::{TcpListener, TcpStream}, thread
};

use flate2::{write::GzEncoder, Compression};

struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(|| handle_connection(s));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn parse_dir() -> String {
    let mut dir: String = "".into();
    for (i, arg) in std::env::args().enumerate() {
        if arg == "--directory" {
            dir = std::env::args().nth(i + 1).unwrap();
        }
    }
    dir
}

fn handle_connection(mut s: TcpStream) {
    println!("accepted new connection");
    let ok_response = "HTTP/1.1 200 OK\r\n\r\n";
    let not_found_response = "HTTP/1.1 404 Not Found\r\n\r\n";
    let buf: &mut [u8] = &mut [0; 1024];
    s.read(buf).unwrap();
    let buf_vec: Vec<u8> = buf.to_vec();
    let str_req = String::from_utf8_lossy(&buf_vec).to_string();
    println!("str req: {}", str_req);
    let req = parse_request(str_req);
    println!("method: {}", req.method);
    println!("path: {}", req.path);
    if req.method == "GET" {
        if req.path == "/" {
            s.write(ok_response.as_bytes()).unwrap();
        } else if req.path.starts_with("/echo/") {
            let echo = extract_suffix(req.path, "/echo/");
            let mut body = echo.to_owned().as_bytes().to_vec();
            let mut headers = vec!["Content-Type: text/plain"];
            if req.headers.get("accept-encoding").unwrap_or(&String::from("")).contains("gzip") {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(echo.as_bytes()).unwrap();
                body = encoder.finish().unwrap();
                headers.push("Content-Encoding: gzip");
            }
            write_response(s, "200 OK", &headers, body);
        } else if req.path == "/user-agent" {
            let value = req.headers.get("user-agent").unwrap();
            write_response(s, "200 OK" , &vec!["Content-Type: text/plain"], value.as_bytes().to_vec());
        } else if req.path.starts_with("/files/") {
            let directory = parse_dir();
            let file = extract_suffix(req.path, "/files/");
            let path = format!("{}/{}", directory, file);
            match fs::read_to_string(path) {
                Ok(f) => {
                    write_response(s, "200 OK" , &vec!["Content-Type: application/octet-stream"], f.as_bytes().to_vec());
                }
                Err(e) => {
                    print!("error: {}", e);
                    s.write(not_found_response.as_bytes()).unwrap();
                }
            }
        } else {
            s.write(not_found_response.as_bytes()).unwrap();
        }
    } else if req.method == "POST" {
      if req.path.starts_with("/files/") {
          let directory = parse_dir();
          let file_name = extract_suffix(req.path, "/files/");
          let path = format!("{}/{}", directory, file_name);
          fs::write(path, req.body.trim_end_matches(char::from(0))).unwrap();
          write_response(s, "201 Created", &vec![], vec![]);
      } else {
        s.write(not_found_response.as_bytes()).unwrap();
      }
    } else {
        s.write(not_found_response.as_bytes()).unwrap();
    }
}

fn write_response(mut s: TcpStream, status: &str, headers: &Vec<&str>, body: Vec<u8>) {
    let mut response = format!( "HTTP/1.1 {}\r\n", status).to_owned();

    for header in headers.into_iter() {
        response.push_str(header);
        response.push_str("\r\n");
    }

    let mut resb = Vec::from(response.as_bytes());

    if body.len() > 0 {
        resb.extend(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
        resb.extend(body.as_slice());
    } else {
        resb.extend("\r\n".as_bytes());
    }

    s.write(resb.as_slice()).unwrap();
}

fn extract_suffix(str: String, prefix: &str) -> String {
    let echo = str.strip_prefix(prefix).unwrap();
    return echo.into();
}

fn parse_request(request: String) -> Request {
    let mut req_split = request.split("\r\n\r\n");
    let path_headers = req_split.next().unwrap();
    let body = req_split.next().unwrap().into();

    let mut path_headers_split = path_headers.split("\r\n");
    let method_path = path_headers_split.next().unwrap();
    let mut method_path_split = method_path.split(" ");

    let method = method_path_split.next().unwrap().into();
    let path = method_path_split.next().unwrap().into();

    let mut headers: HashMap<String, String> = HashMap::new();

    for header in path_headers_split {
        let mut header_iter = header.split(": ");
        let key = header_iter.next().unwrap().to_lowercase();
        let value = header_iter.next().unwrap().to_lowercase();
        println!("header: {}", header);
        println!("key: {}", key);
        println!("value: {}", value);
        headers.insert(String::from(key), String::from(value));
    }

    Request {
        method,
        path,
        headers,
        body,
    }
}
