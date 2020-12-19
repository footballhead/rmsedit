pub struct Monster {
    gfx_id: u8, // TODO other fields
}

const MONSTER_RECORD_SIZE: usize = 0x1F;
const MONSTER_RECORD_GFX_ID_OFFEST: usize = 0x16;

pub fn load_monsters(filename: &str) -> Vec<Monster> {
    let mon_dat = std::fs::read(filename).unwrap();
    return mon_dat
        .chunks(MONSTER_RECORD_SIZE)
        .map(|x| Monster {
            gfx_id: x[MONSTER_RECORD_GFX_ID_OFFEST],
        })
        .collect();
}
