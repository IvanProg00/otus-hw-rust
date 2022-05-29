use intelligent_house_2::House;

fn main() {
    let mut h = House::new(String::from("house 1"));
    println!("House name: {}", h.get_name());

    h.push_room(String::from("room 1")).unwrap();
    h.push_room(String::from("room 2")).unwrap();

    for r in h.list_rooms().iter() {
        println!("Room name: {}", r.get_name());
    }

    let mut r = h.list_rooms()[0];
    r.push_device(String::from("device 1")).unwrap();
}
