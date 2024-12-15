use std::{
    io::{Read, Write},
    net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener},
};

#[tauri::command]
pub async fn start_server() -> Result<(), String> {
    let port_number = 33117;
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port_number))
        .map_err(|e| format!("Failed to bind to port: {}", e))?;

    println!(
        "Listening on  {}:{}",
        localip::get_local_ip().map_err(|e| format!("{}", e))?,
        port_number
    );

    match listener.accept() {
        Ok((mut socket, addr)) => {
            println!("Accepted a connection from: {}", addr);
            let mut buffer = [0; 1024];
            match socket.read(&mut buffer) {
                Ok(bytes_read) => {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Received message is \"{}\"", message);
                }
                Err(e) => {
                    return Err(format!("Error reading from client: {}", e));
                }
            }

            let response = "Hello from server";
            if let Err(e) = socket.write_all(response.as_bytes()) {
                return Err(format!("Error sending message to client: {}", e));
            }
            println!("Message sent to client");

            // Close the connection.
            socket
                .shutdown(Shutdown::Both)
                .map_err(|e| format!("Failed to close the connection: {}", e))?;
        }
        Err(e) => {
            println!("Failed to accept a connection: {}", e);
            return Err("Failed to accept a connection".into());
        }
    }

    Ok(())
}
