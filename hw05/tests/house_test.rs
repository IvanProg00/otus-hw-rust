use intelligent_house_with_tests::intelligent::{house::House, house::IntelligentHouse};

#[test]
fn test_house_new() {
    let h: House = IntelligentHouse::new(String::from("house 1"));

    assert_eq!(h.get_name(), "house 1");
    assert!(h.list_rooms().is_empty());
}

#[test]
fn test_house_new_with_rooms() {
    let mut h: House = IntelligentHouse::new(String::from("house 2"));

    assert!(h.push_room(String::from("room 1")).is_ok());
    assert!(h.push_room(String::from("room 2")).is_ok());
    assert!(h.push_room(String::from("room 3")).is_ok());

    assert_eq!(h.get_name(), "house 2");

    let rooms = h.list_rooms();
    assert_eq!(rooms.len(), 3);

    assert_eq!(rooms.get(0).unwrap().get_name(), "room 1");
    assert_eq!(rooms.get(1).unwrap().get_name(), "room 2");
    assert_eq!(rooms.get(2).unwrap().get_name(), "room 3");
}

#[test]
fn test_house_new_with_devices() {
    let mut h: House = IntelligentHouse::new(String::from("house 2"));

    assert!(h.push_room(String::from("room 1")).is_ok());

    assert_eq!(h.get_name(), "house 2");

    let opt = h.get_room_mut(0);
    assert!(opt.is_some());

    let room = opt.unwrap();
    assert_eq!(room.get_name(), "room 1");

    assert!(room.push_device(String::from("device 2")).is_ok());
    assert!(room.push_device(String::from("device 1")).is_ok());
    assert!(room.push_device(String::from("device 3")).is_ok());
    assert!(room.push_device(String::from("device 4")).is_ok());

    let devices = room.list_devices();
    assert_eq!(devices.len(), 4);

    assert_eq!(devices.get(0).unwrap().get_name(), "device 2");
    assert_eq!(devices.get(1).unwrap().get_name(), "device 1");
    assert_eq!(devices.get(2).unwrap().get_name(), "device 3");
    assert_eq!(devices.get(3).unwrap().get_name(), "device 4");
}

#[test]
fn test_create_report() {
    let mut h: House = IntelligentHouse::new(String::from("house 2"));

    assert!(h.push_room(String::from("room 1")).is_ok());
    assert!(h.push_room(String::from("room 2")).is_ok());

    assert_eq!(h.get_name(), "house 2");

    {
        let room_1 = h.get_room_mut(0).unwrap();
        assert_eq!(room_1.get_name(), "room 1");

        assert!(room_1.push_device(String::from("device 4")).is_ok());
        assert!(room_1.push_device(String::from("device 2")).is_ok());
        assert!(room_1.push_device(String::from("device 1")).is_ok());
        assert!(room_1.push_device(String::from("device 3")).is_ok());
    }

    {
        let room_2 = h.get_room_mut(1).unwrap();
        assert_eq!(room_2.get_name(), "room 2");

        assert!(room_2.push_device(String::from("device 9")).is_ok());
        assert!(room_2.push_device(String::from("device 8")).is_ok());
        assert!(room_2.push_device(String::from("device 6")).is_ok());
        assert!(room_2.push_device(String::from("device 7")).is_ok());
    }

    assert_eq!(
        h.get_room(0).unwrap().get_device(0).unwrap().get_name(),
        "device 4"
    );
    assert_eq!(
        h.get_room(0).unwrap().get_device(1).unwrap().get_name(),
        "device 2"
    );
    assert_eq!(
        h.get_room(0).unwrap().get_device(2).unwrap().get_name(),
        "device 1"
    );
    assert_eq!(
        h.get_room(0).unwrap().get_device(3).unwrap().get_name(),
        "device 3"
    );

    assert_eq!(
        h.get_room(1).unwrap().get_device(0).unwrap().get_name(),
        "device 9"
    );
    assert_eq!(
        h.get_room(1).unwrap().get_device(1).unwrap().get_name(),
        "device 8"
    );
    assert_eq!(
        h.get_room(1).unwrap().get_device(2).unwrap().get_name(),
        "device 6"
    );
    assert_eq!(
        h.get_room(1).unwrap().get_device(3).unwrap().get_name(),
        "device 7"
    );

    assert_eq!(
        h.create_report(),
        String::from(
            "House \"house 2\":
\tRoom \"room 1\":
\t\tDevice \"device 4\"
\t\tDevice \"device 2\"
\t\tDevice \"device 1\"
\t\tDevice \"device 3\"
\tRoom \"room 2\":
\t\tDevice \"device 9\"
\t\tDevice \"device 8\"
\t\tDevice \"device 6\"
\t\tDevice \"device 7\""
        )
    )
}
