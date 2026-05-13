use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

pub const GWORLD_OFFSET: usize = 0x58BE190;

pub const ACTOR_ROOT_COMPONENT_OFFSET: usize = 0x1A0;
pub const COMPONENT_LOCATION_OFFSET: usize = 0x1D8;

pub static CANDIDATE_LEVEL_OFFSETS: [usize; 3] = [0x30, 0x38, 0x150];
pub static CANDIDATE_ACTORS_OFFSETS: [usize; 4] = [0x98, 0xA0, 0xA8, 0xB0];

pub fn get_module_base() -> usize {
    unsafe {
        match GetModuleHandleA(PCSTR::null()) {
            Ok(h) => h.0 as usize,
            Err(_) => 0,
        }
    }
}

unsafe fn read_ptr(addr: usize) -> usize {
    if addr < 0x10000 {
        return 0;
    }
    *(addr as *const usize)
}

unsafe fn read_i32(addr: usize) -> i32 {
    if addr < 0x10000 {
        return 0;
    }
    *(addr as *const i32)
}

pub fn get_gworld(base: usize) -> usize {
    if base == 0 {
        return 0;
    }
    unsafe { read_ptr(base + GWORLD_OFFSET) }
}

pub struct ActorArray {
    pub data: usize,
    pub count: i32,
}

pub fn scan_offsets(world: usize) -> [(usize, usize, i32); 12] {
    let mut results = [(0usize, 0usize, 0i32); 12];
    if world == 0 {
        return results;
    }
    let mut idx = 0;
    unsafe {
        for &lv_off in &CANDIDATE_LEVEL_OFFSETS {
            let level = read_ptr(world + lv_off);
            for &arr_off in &CANDIDATE_ACTORS_OFFSETS {
                if idx >= 12 {
                    break;
                }
                if level < 0x10000 {
                    results[idx] = (lv_off, arr_off, -1);
                } else {
                    let count = read_i32(level + arr_off + 8);
                    results[idx] = (lv_off, arr_off, count);
                }
                idx += 1;
            }
        }
    }
    results
}

pub fn find_best_actors(world: usize) -> (usize, ActorArray) {
    if world == 0 {
        return (0, ActorArray { data: 0, count: 0 });
    }
    unsafe {
        for &lv_off in &CANDIDATE_LEVEL_OFFSETS {
            let level = read_ptr(world + lv_off);
            if level < 0x10000 {
                continue;
            }
            for &arr_off in &CANDIDATE_ACTORS_OFFSETS {
                let data = read_ptr(level + arr_off);
                let count = read_i32(level + arr_off + 8);
                if data > 0x10000 && count > 0 && count < 50_000 {
                    return (level, ActorArray { data, count });
                }
            }
        }
        (0, ActorArray { data: 0, count: 0 })
    }
}

pub fn get_actor(array: &ActorArray, index: i32) -> usize {
    if array.data == 0 || index < 0 || index >= array.count {
        return 0;
    }
    unsafe { read_ptr(array.data + (index as usize * 8)) }
}

pub fn get_actor_location(actor: usize) -> Option<[f32; 3]> {
    if actor == 0 {
        return None;
    }
    unsafe {
        let root = read_ptr(actor + ACTOR_ROOT_COMPONENT_OFFSET);
        if root < 0x10000 {
            return None;
        }
        let loc_ptr = (root + COMPONENT_LOCATION_OFFSET) as *const [f32; 3];
        Some(*loc_ptr)
    }
}
