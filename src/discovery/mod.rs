use std::{io, str, time::Duration};
use tokio::{net::UdpSocket, time::timeout};

pub async fn discover_inverters() -> io::Result<()> {
    println!("Trying to discover GoodWe inverters...");
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let remote_addr = "255.255.255.255:48899";
    let request = "WIFIKIT-214028-READ";

    sock.set_broadcast(true)?;
    let _ = sock.send_to(&request.as_bytes(), remote_addr).await?;
    println!("Sent discovery trigger");

    let mut buf = [0; 1024];
    let mut found_inverters = 0;

    loop {
        match timeout(Duration::from_secs(5), sock.recv_from(&mut buf)).await {
            Err(_) => {
                if found_inverters == 0 {
                    println!("Could not find any inverters");
                    return Ok(());
                }

                println!("\nFound {} inverters", found_inverters);
                return Ok(());
            }
            Ok(Err(e)) => {
                println!("Problem receiving data: {}", e);
                return Err(e);
            }
            Ok(Ok((_, addr))) => {
                found_inverters += 1;
                let end = buf.into_iter().position(|x| x == b'\0').unwrap();
                let bufstring = str::from_utf8(&buf[0..end]).unwrap();

                let items: Vec<&str> = bufstring.split_terminator(',').collect();
                println!("{}: Discovered inverter at {}:", found_inverters, addr.ip());
                println!("\t- IP Address: {}", items[0]);
                println!("\t- Serial Number: {}", items[1]);
                println!("\t- WiFi Name: {}", items[2]);
            }
        }
    }
}
