use std::io::Write;
use std::time::Duration;

use serialport::{DataBits, StopBits};

fn main(){
    let portname = "COM3";
    let baudrate = 115200;
    let databits = DataBits::Eight;
    let stopbits = StopBits::One;
    let rate     = 1;
    let string   = "rust";

    let build = serialport::new(portname,baudrate)
        .data_bits(databits)
        .stop_bits(stopbits);
    
    println!("{:?}", build);

    let mut port = build.open().unwrap_or_else(|_e|{
        eprintln!("Failed to Open Port");
        ::std::process::exit(1);
    });

    println!("Writing Data");

    loop{
        match port.write(string.as_bytes()){
            Ok(_) => {
                println!("{}", string);
                std::io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e)=> println!("{:?}", e)
        }

        if rate==0{
            return;
        }

        std::thread::sleep(Duration::from_millis((1000.0/(rate as f32)) as u64));
    }
}