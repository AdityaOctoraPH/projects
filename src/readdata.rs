use std::io::Write;
use std::time::Duration;

fn main(){
    let portname = "COM3";
    let baudrate = 115200;

    let port = serialport::new(portname,baudrate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port)=>{
            let mut serialbuf: Vec<u8> = vec![0;100];
            println!("Reading Data");
            loop{
                match port.read(serialbuf.as_mut_slice()){
                    Ok(t) => std::io::stdout().write_all(&serialbuf[..t]).unwrap(),
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => println!("{:?}", e)
                }
            }
        }

        Err(e)=>{
            println!("{:?}",e);
            ::std::process::exit(1);
        }
    }
}