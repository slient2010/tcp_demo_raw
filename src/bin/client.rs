use std::io;
// use std::io::Read;
use std::net::{SocketAddr, TcpStream};

use structopt::StructOpt;

use tcp_demo_raw::{extract_string_unbuffered, write_data, DEFAULT_SERVER_ADDR};

#[derive(Debug, StructOpt)]
#[structopt(name = "client")]
struct Args {
    message: String,
    /// Server destination address
    #[structopt(long, default_value = DEFAULT_SERVER_ADDR, global = true)]
    addr: SocketAddr,
}

fn main() -> io::Result<()> {
    let args = Args::from_args();

    let mut stream = TcpStream::connect(args.addr)?;
    let send_server = &args.message.as_bytes();
    loop {
        write_data(&mut stream, send_server)?;

        // let mut data = vec![0; 256];
        // let read_data = stream.read(&mut data)?;
        // let k = data.clone();
        // println!("-..{}. > {}", read_data, String::from_utf8(k).unwrap());

        // Now read & print the response
        // (this will block until all data has been received)
        let _ = extract_string_unbuffered(&mut stream)
            .map(|resp| println!("data from server -->{}", resp));
        use std::{thread, time};
        let ten_millis = time::Duration::from_millis(1000);

        thread::sleep(ten_millis);
    }
}
