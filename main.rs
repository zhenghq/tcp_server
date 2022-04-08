use std::thread; 
use std::net::{TcpListener, TcpStream, Shutdown}; 
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // 设置buff
    while match stream.read(&mut data) { //无限循环 使用match匹配结果
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap(); 
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap()); //如果失败抛出错误
            stream.shutdown(Shutdown::Both).unwrap(); // 关闭流
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap(); //listen port 3333
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() { // 遍历stream流
        match stream {
            Ok(stream) => {  // 如果获取stream成功
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // 连接成功
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
