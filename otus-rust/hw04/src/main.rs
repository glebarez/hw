use std::collections::HashMap;

struct _SmartHome {
    name: String,
    rooms: HashMap<String, _Room>,
}

impl _SmartHome {
    fn _get_rooms(&self) -> Vec<_Room> {
        todo!()
    }
    fn _add_room(_name: String, _room: _Room) {
        todo!()
    }
    fn _remove_room(_name: String) {
        todo!()
    }
}

struct _Room {
    name: String,
    devices: HashMap<String, _Device>,
}

impl _Room {
    fn _add_device(_name: String, _device: _Device) {
        todo!()
    }
    fn _remove_device(_name: String) {
        todo!()
    }
}

enum _Device {
    Thermo(_Thermo),
    Socket(_Socket),
}

struct _Thermo;

impl _Thermo {
    fn _get_temperature(&self) -> f32 {
        todo!()
    }
}

struct _Socket;

impl _Socket {
    fn _turn_on(&self) {
        todo!()
    }
    fn _turn_off(&self) {
        todo!()
    }
    fn _get_power_consumption(&self) -> f32 {
        todo!()
    }
}

fn main() {
    println!("Smart home!");
}
