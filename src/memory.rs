use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

pub const GAME_EXE: &str = "Greylock-Win64-Shipping.exe";

pub const GWORLD_OFFSET: usize = 0x58BE190;

pub const PERSISTENT_LEVEL_OFFSET: usize = 0x30;
pub const ACTORS_ARRAY_OFFSET: usize = 0xA0;
pub const ACTORS_COUNT_OFFSET: usize = 0xA8;

pub const ACTOR_ROOT_COMPONENT_OFFSET: usize = 0x1A0;
pub const COMPONENT_LOCATION_OFFSET: usize = 0x1D8;

pub fn get_module_base() -> usize {
    unsafe {
        let handle = GetModuleHandleA(PCSTR::null());
        match handle {
            Ok(h) => h.0 as usize,
            Err(_) => 0,
        }
    }
}

pub fn get_gworld(base: usize) -> usize {
    if base == 0 {
        return 0;
    }
    unsafe {
        let ptr = (base + GWORLD_OFFSET) as *const usize;
        if ptr.is_null() {
            return 0;
        }
        *ptr
    }
}

pub fn get_persistent_level(world: usize) -> usize {
    if world == 0 {
        return 0;
    }
    unsafe {
        let ptr = (world + PERSISTENT_LEVEL_OFFSET) as *const usize;
        *ptr
    }
}

pub struct ActorArray {
    pub data: usize,
    pub count: i32,
}

pub fn get_actors(level: usize) -> ActorArray {
    if level == 0 {
        return ActorArray { data: 0, count: 0 };
    }
    unsafe {
        let data_ptr = (level + ACTORS_ARRAY_OFFSET) as *const usize;
        let count_ptr = (level + ACTORS_COUNT_OFFSET) as *const i32;
        ActorArray {
            data: *data_ptr,
            count: *count_ptr,
        }
    }
}

pub fn get_actor(array: &ActorArray, index: i32) -> usize {
    if array.data == 0 || index < 0 || index >= array.count {
        return 0;
    }
    unsafe {
        let ptr = (array.data + (index as usize * 8)) as *const usize;
        *ptr
    }
}

pub fn get_actor_location(actor: usize) -> Option<[f32; 3]> {
    if actor == 0 {
        return None;
    }
    unsafe {
        let root_ptr = (actor + ACTOR_ROOT_COMPONENT_OFFSET) as *const usize;
        let root = *root_ptr;
        if root == 0 {
            return None;
        }
        let loc_ptr = (root + COMPONENT_LOCATION_OFFSET) as *const [f32; 3];
        Some(*loc_ptr)
    }
}
