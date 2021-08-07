use serialport::{available_ports, SerialPortType};

fn main(){
    match available_ports(){
        Ok(ports)=>{
            match ports.len(){
                0 => println!("No Port Found"),
                1 => println!("1 Port Found"),
                _ => println!("Many Ports Found")
            }

            for port in ports{
                println!("  {}", port.port_name);
                match port.port_type{
                    SerialPortType::UsbPort(_usbinfo)=>{
                        println!("  Usb Type");
                    }

                    SerialPortType::BluetoothPort=>{
                        println!("  Bluetooth Type");
                    }

                    SerialPortType::PciPort=>{
                        println!("  Pci Type");
                    }

                    SerialPortType::Unknown=>{
                        println!("  Unknown Type");
                    }
                }
            }
        }

        Err(e)=>{
            println!("{:?}",e);
        }
    }
}
