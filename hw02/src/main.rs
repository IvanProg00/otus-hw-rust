use intelligent_house::{SmartSocket, Thermometer};

fn main() {
    let smart_socket = SmartSocket::new("F489341");
    println!("About: {}", smart_socket.about());

    let thermometer = Thermometer::new();
    println!("Current temperature: {}", thermometer.get_temperature());
}
