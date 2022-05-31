use intelligent_house_2::House;

fn main() {
    let mut h = House::new(String::from("house 1"));
    println!("House name: {}", h.get_name());

    h.push_room(String::from("room 1")).unwrap();
    h.push_room(String::from("room 2")).unwrap();

    for r in h.list_rooms().iter() {
        println!("Room name: {}", r.get_name());
    }

    let room1 = match h.list_rooms_mut().get_mut(0) {
        Some(r) => r,
        None => panic!("Room not found"),
    };
    room1.push_device(String::from("device 1")).unwrap();

    for d in room1.list_devices() {
        println!(
            "Room name: {}; Device name: {}",
            room1.get_name(),
            d.get_name()
        );
    }

    let room2 = match h.get_room_mut(1) {
        Some(r) => r,
        None => panic!("Room not found"),
    };

    for i in 1..6 {
        room2.push_device(format!("device {}", i)).unwrap();
    }

    for d in room2.list_devices().iter() {
        println!(
            "Room name: {}; Device name: {}",
            room2.get_name(),
            d.get_name()
        );
    }

    room2.push_device(String::from("repeat name")).unwrap();
    match room2.push_device(String::from("repeat name")) {
        Ok(_) => panic!("Expected panic"),
        Err(e) => println!("Received error: {}", e),
    }

    h.push_room(String::from("room 3")).unwrap();

    println!("--------------------");
    println!("{}", h.get_report());
}
