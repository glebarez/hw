#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::collections::HashMap;

struct _SmartHome {
    name: String,
    rooms: HashMap<String, _Room>,
}

struct _AddResult;
struct _RemoveResult;
struct _ActionResult;
struct _SmartHomeReport;

impl _SmartHome {
    fn _get_rooms(&self) -> Vec<_Room> {
        todo!()
    }
    fn _add_room(&self, _name: String, _room: _Room) -> _AddResult {
        todo!()
    }
    fn _remove_room(&self, _name: String) -> _RemoveResult {
        todo!()
    }
    fn _build_report(&self) -> _SmartHomeReport {
        todo!()
    }
}

struct _Room {
    name: String,
    devices: HashMap<String, _Device>,
}

impl _Room {
    fn _add_device(&self, _name: String, _device: _Device) -> _AddResult {
        todo!()
    }
    fn _remove_device(&self, _name: String) -> _RemoveResult {
        todo!()
    }
    fn _get_device(&self, _name: String) -> _Device {
        todo!()
    }
    fn _get_devices(&self) -> Vec<_Device> {
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
    fn _turn_on(&self) -> _ActionResult {
        todo!()
    }
    fn _turn_off(&self) -> _ActionResult {
        todo!()
    }
    fn _get_power_consumption(&self) -> f32 {
        todo!()
    }
}
