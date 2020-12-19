pub const ROOM_WIDTH: u32 = 20;
pub const ROOM_HEIGHT: u32 = 8;

const ROOM_AREA: usize = (ROOM_WIDTH * ROOM_HEIGHT) as usize;

const ROOM_RECORD_SIZE: usize = 0x168;
const ROOM_RECORD_TILE_OFFSET: usize = 0x1;
const ROOM_RECORD_OBJECT_OFFSET: usize = 0xA1;
const ROOM_RECORD_MONSTER_ID_OFFSET: usize = 0x141;
const ROOM_RECORD_MONSTER_COUNT_OFFSET: usize = 0x142;
const ROOM_RECORD_NORTH_OFFSET: usize = 0x143;
const ROOM_RECORD_ID_OFFSET: usize = 0x149;

pub struct Room {
    unknown_a: u8,
    tiles: [u8; ROOM_AREA],
    objects: [u8; ROOM_AREA],
    monster_id: u8,
    monster_count: u8,
    nav_north: u8,
    nav_east: u8,
    nav_south: u8,
    nav_west: u8,
    nav_up: u8,
    nav_down: u8,
    id: u8,
    unknown_b: u8,
    unknown_c: u8,
    unknown_d: u8,
    name: String,
}

impl Room {
    /// The null tile is 0 and should not be drawn, so don't forget to -1 the return value.
    pub fn get_tile(&self, x: u32, y: u32) -> u8 {
        // TODO: Panic if x or y out of bounds
        let tile = self.tiles[(y * ROOM_WIDTH + x) as usize];
        // Different traps are different ASCII characters, which is what > 84 catches
        return if tile > 84 { 21 } else { tile };
    }
}

pub fn load_rooms(filename: &str) -> Vec<Room> {
    let rms_data = std::fs::read(filename).unwrap();
    return rms_data
        .chunks(ROOM_RECORD_SIZE)
        .map(|x| {
            let mut room = Room {
                unknown_a: 0,
                tiles: [0; ROOM_AREA],
                objects: [0; ROOM_AREA],
                monster_id: x[ROOM_RECORD_MONSTER_ID_OFFSET],
                monster_count: x[ROOM_RECORD_MONSTER_COUNT_OFFSET],
                nav_north: x[ROOM_RECORD_NORTH_OFFSET],
                nav_east: x[ROOM_RECORD_NORTH_OFFSET + 1],
                nav_south: x[ROOM_RECORD_NORTH_OFFSET + 2],
                nav_west: x[ROOM_RECORD_NORTH_OFFSET + 3],
                nav_up: x[ROOM_RECORD_NORTH_OFFSET + 4],
                nav_down: x[ROOM_RECORD_NORTH_OFFSET + 5],
                id: x[ROOM_RECORD_ID_OFFSET],
                unknown_b: 0,
                unknown_c: 0,
                unknown_d: 0,
                name: String::from("UNIMPLEMENTED"),
            };
            room.tiles
                .copy_from_slice(&x[ROOM_RECORD_TILE_OFFSET..ROOM_RECORD_TILE_OFFSET + ROOM_AREA]);
            room.objects.copy_from_slice(
                &x[ROOM_RECORD_OBJECT_OFFSET..ROOM_RECORD_OBJECT_OFFSET + ROOM_AREA],
            );
            room
        })
        .collect();
}
