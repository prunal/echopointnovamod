use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use std::mem;

pub const GAME_EXE: &str = "Greylock-Win64-Shipping.exe";
pub const GWORLD_OFFSET: usize = 0x58BE190;
pub const GNAMES_OFFSET: usize = 0x573ACC0;
pub const GOBJECTS_OFFSET: usize = 0x5777010;

pub struct MemoryReader {
    pub base_address: usize,
    pub process_handle: usize,
}

impl MemoryReader {
    pub fn new(_process_id: u32) -> Option<Self> {
        Some(MemoryReader {
            base_address: 0,
            process_handle: 0,
        })
    }

    pub fn read<T: Copy>(&self, address: usize) -> Option<T> {
        unsafe {
            let value = *(address as *const T);
            Some(value)
        }
    }

    pub fn write<T: Copy>(&self, address: usize, value: T) -> bool {
        unsafe {
            *(address as *mut T) = value;
            true
        }
    }

    pub fn read_string(&self, address: usize, max_len: usize) -> Option<String> {
        unsafe {
            let bytes = std::slice::from_raw_parts(address as *const u8, max_len);
            let end = bytes.iter().position(|&b| b == 0).unwrap_or(max_len);
            String::from_utf8_lossy(&bytes[..end]).to_string().into()
        }
    }

    pub fn find_process_id(exe_name: &str) -> Option<u32> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;

            let mut entry: PROCESSENTRY32 = mem::zeroed();
            entry.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

            Process32First(snapshot, &mut entry).ok()?;

            loop {
                let name = String::from_utf8_lossy(&entry.szExeFile[..].iter().map(|&c| c as u8).collect::<Vec<u8>>())
                    .trim_end_matches('\0')
                    .to_string();

                if name.eq_ignore_ascii_case(exe_name) {
                    return Some(entry.th32ProcessID);
                }

                if Process32Next(snapshot, &mut entry).is_err() {
                    break;
                }
            }

            None
        }
    }
}

pub fn get_player() -> Option<usize> {
    unsafe {
        let world = *(GWORLD_OFFSET as *const usize);
        if world == 0 {
            return None;
        }

        // GWorld -> PersistentLevel (offset varies, simplified)
        let persistent_level = *(world.wrapping_add(0x30) as *const usize);
        if persistent_level == 0 {
            return None;
        }

        // Simplified: scan for GLBaseCharacter with known offsets
        Some(persistent_level)
    }
}

pub fn read_player_hp(player: usize) -> Option<f32> {
    unsafe {
        let hp_addr = player.wrapping_add(0x564);
        Some(*(hp_addr as *const f32))
    }
}

pub fn write_player_hp(player: usize, value: f32) {
    unsafe {
        let hp_addr = player.wrapping_add(0x564) as *mut f32;
        *hp_addr = value;
    }
}

pub fn read_max_hp(player: usize) -> Option<f32> {
    unsafe {
        let max_hp_addr = player.wrapping_add(0x568);
        Some(*(max_hp_addr as *const f32))
    }
}

pub fn write_invincible(player: usize, value: bool) {
    unsafe {
        let invincible_addr = player.wrapping_add(0x56C) as *mut bool;
        *invincible_addr = value;
    }
}

pub fn read_is_dead(player: usize) -> Option<bool> {
    unsafe {
        let dead_addr = player.wrapping_add(0x56D);
        Some(*(dead_addr as *const bool))
    }
}

pub fn get_story_save() -> Option<usize> {
    unsafe {
        let world = *(GWORLD_OFFSET as *const usize);
        if world == 0 {
            return None;
        }

        // Simplified offset to story save singleton
        Some(world.wrapping_add(0x100))
    }
}

pub fn read_cash(story_save: usize) -> Option<i32> {
    unsafe {
        let cash_addr = story_save.wrapping_add(0xF8);
        Some(*(cash_addr as *const i32))
    }
}

pub fn write_cash(story_save: usize, value: i32) {
    unsafe {
        let cash_addr = story_save.wrapping_add(0xF8) as *mut i32;
        *cash_addr = value;
    }
}

pub fn read_energy(story_save: usize) -> Option<f32> {
    unsafe {
        let energy_addr = story_save.wrapping_add(0x4C4);
        Some(*(energy_addr as *const f32))
    }
}

pub fn write_energy(story_save: usize, value: f32) {
    unsafe {
        let energy_addr = story_save.wrapping_add(0x4C4) as *mut f32;
        *energy_addr = value;
    }
}

pub fn read_agility_orbs(story_save: usize) -> Option<i32> {
    unsafe {
        let orbs_addr = story_save.wrapping_add(0x598);
        Some(*(orbs_addr as *const i32))
    }
}

pub fn write_agility_orbs(story_save: usize, value: i32) {
    unsafe {
        let orbs_addr = story_save.wrapping_add(0x598) as *mut i32;
        *orbs_addr = value;
    }
}

pub fn read_workshop_orbs(story_save: usize) -> Option<i32> {
    unsafe {
        let orbs_addr = story_save.wrapping_add(0x59C);
        Some(*(orbs_addr as *const i32))
    }
}

pub fn write_workshop_orbs(story_save: usize, value: i32) {
    unsafe {
        let orbs_addr = story_save.wrapping_add(0x59C) as *mut i32;
        *orbs_addr = value;
    }
}

pub fn get_player_state() -> Option<usize> {
    unsafe {
        let world = *(GWORLD_OFFSET as *const usize);
        if world == 0 {
            return None;
        }

        // Simplified offset to player state
        Some(world.wrapping_add(0x200))
    }
}

pub fn read_kills_this_round(player_state: usize) -> Option<i32> {
    unsafe {
        let kills_addr = player_state.wrapping_add(0xB18);
        Some(*(kills_addr as *const i32))
    }
}
