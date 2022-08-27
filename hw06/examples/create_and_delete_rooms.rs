use intelligent_house_with_tests::intelligent::{house::House, house::IntelligentHouse};

fn main() {
    let mut h: House = IntelligentHouse::new(String::from("house 1"));
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

    println!("Before delete rooms");
    println!("rooms number: {}", h.list_rooms().len());

    h.delete_room(1).unwrap();
    h.delete_room(0).unwrap();

    println!("After delete rooms");
    println!("rooms number: {}", h.list_rooms().len());
}
