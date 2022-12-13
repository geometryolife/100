use mylib::ThreadPool;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // Conditionally return to a web page
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    let te = time::Duration::from_millis(10000);
    thread::sleep(te); // Sleep for a while, simulation processing time is long
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?; // `?` means that the returned type is Result
                                                         // let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    let pool = ThreadPool::new(4);

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Create a thread
        // let handle = thread::spawn(move || {
        //     handle_client(stream);
        // });

        // thread_vec.push(handle);

        // Thread pool
        pool.execute(|| handle_client(stream));
    }

    // for handle in thread_vec {
    //     handle.join().unwrap();
    // }

    Ok(())
}

// 当前存在的问题：
// 当存在海量请求时，系统也会跟着创建海量的线程，最终造成系统崩溃。
