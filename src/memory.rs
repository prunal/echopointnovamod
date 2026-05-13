use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{
    VirtualQuery, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_GUARD, PAGE_NOACCESS,
};

pub const GWORLD_OFFSET: usize = 0x58BE190;

pub const PERSISTENT_LEVEL_OFFSET: usize = 0x30;
pub const ACTORS_ARRAY_OFFSET: usize = 0x98;

pub const ACTOR_ROOT_COMPONENT_OFFSET: usize = 0x1A0;
pub const COMPONENT_LOCATION_OFFSET: usize = 0x1D8;

pub const GAME_INSTANCE_OFFSET: usize = 0x180;
pub const LOCAL_PLAYERS_OFFSET: usize = 0x38;
pub const PLAYER_CONTROLLER_OFFSET: usize = 0x30;
pub const CAMERA_MANAGER_OFFSET: usize = 0x348;

pub const POV_CANDIDATE_COUNT: usize = 6;

pub fn get_module_base() -> usize {
    unsafe {
        match GetModuleHandleA(PCSTR::null()) {
            Ok(h) => h.0 as usize,
            Err(_) => 0,
        }
    }
}

fn is_readable(addr: usize, size: usize) -> bool {
    if addr < 0x10000 || addr > 0x7FFF_FFFF_FFFF {
        return false;
    }
    unsafe {
        let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
        let written = VirtualQuery(
            Some(addr as *const _),
            &mut mbi,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        );
        if written == 0 || mbi.State != MEM_COMMIT {
            return false;
        }
        let bad = PAGE_NOACCESS.0 | PAGE_GUARD.0;
        if mbi.Protect.0 & bad != 0 {
            return false;
        }
        let region_end = (mbi.BaseAddress as usize).saturating_add(mbi.RegionSize);
        if addr.saturating_add(size) > region_end {
            return false;
        }
        true
    }
}

fn safe_read_ptr(addr: usize) -> usize {
    if !is_readable(addr, 8) { return 0; }
    unsafe { *(addr as *const usize) }
}

fn safe_read_i32(addr: usize) -> i32 {
    if !is_readable(addr, 4) { return 0; }
    unsafe { *(addr as *const i32) }
}

fn safe_read_f32(addr: usize) -> Option<f32> {
    if !is_readable(addr, 4) { return None; }
    let v = unsafe { *(addr as *const f32) };
    if v.is_finite() { Some(v) } else { None }
}

fn safe_read_vec3(addr: usize) -> Option<[f32; 3]> {
    if !is_readable(addr, 12) { return None; }
    let v = unsafe { *(addr as *const [f32; 3]) };
    if v[0].is_finite() && v[1].is_finite() && v[2].is_finite() {
        Some(v)
    } else {
        None
    }
}

pub fn get_gworld(base: usize) -> usize {
    if base == 0 { return 0; }
    safe_read_ptr(base + GWORLD_OFFSET)
}

pub struct ActorArray {
    pub data: usize,
    pub count: i32,
}

pub fn get_actors(world: usize) -> (usize, ActorArray) {
    if world == 0 {
        return (0, ActorArray { data: 0, count: 0 });
    }
    let level = safe_read_ptr(world + PERSISTENT_LEVEL_OFFSET);
    if level == 0 {
        return (0, ActorArray { data: 0, count: 0 });
    }
    let data = safe_read_ptr(level + ACTORS_ARRAY_OFFSET);
    let count = safe_read_i32(level + ACTORS_ARRAY_OFFSET + 8);
    if data == 0 || count <= 0 || count > 50_000 {
        return (level, ActorArray { data: 0, count: 0 });
    }
    (level, ActorArray { data, count })
}

pub fn get_actor(array: &ActorArray, index: i32) -> usize {
    if array.data == 0 || index < 0 || index >= array.count {
        return 0;
    }
    safe_read_ptr(array.data + (index as usize * 8))
}

pub fn get_actor_location(actor: usize) -> Option<[f32; 3]> {
    if actor == 0 { return None; }
    let root = safe_read_ptr(actor + ACTOR_ROOT_COMPONENT_OFFSET);
    if root == 0 { return None; }
    safe_read_vec3(root + COMPONENT_LOCATION_OFFSET)
}

#[derive(Default, Clone, Copy)]
pub struct PovCandidate {
    pub offset: usize,
    pub location: [f32; 3],
    pub rotation: [f32; 3],
    pub fov: f32,
}

pub fn scan_pov_candidates(cm: usize) -> [PovCandidate; POV_CANDIDATE_COUNT] {
    let mut out: [PovCandidate; POV_CANDIDATE_COUNT] = [PovCandidate::default(); POV_CANDIDATE_COUNT];
    if cm == 0 { return out; }

    let mut found: Vec<(f32, PovCandidate)> = Vec::with_capacity(32);

    let mut off = 0x10usize;
    while off < 0x5000 {
        if let Some(fov) = safe_read_f32(cm + off) {
            if fov >= 30.0 && fov <= 170.0 && fov != 0.0 {
                let loc_off = off.wrapping_sub(0x18);
                let rot_off = off.wrapping_sub(0xC);
                if let (Some(loc), Some(rot)) = (
                    safe_read_vec3(cm + loc_off),
                    safe_read_vec3(cm + rot_off),
                ) {
                    let loc_ok = loc[0].abs() < 1.0e7 && loc[1].abs() < 1.0e7 && loc[2].abs() < 1.0e7;
                    let rot_ok = rot[0].abs() <= 720.0 && rot[1].abs() <= 720.0 && rot[2].abs() <= 720.0;
                    let loc_nonzero = loc[0].abs() + loc[1].abs() + loc[2].abs() > 1.0;
                    if loc_ok && rot_ok && loc_nonzero {
                        let dev = (90.0f32 - fov).abs();
                        let cand = PovCandidate {
                            offset: loc_off,
                            location: loc,
                            rotation: rot,
                            fov,
                        };
                        found.push((dev, cand));
                    }
                }
            }
        }
        off += 4;
    }

    found.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    for (i, (_, c)) in found.iter().take(POV_CANDIDATE_COUNT).enumerate() {
        out[i] = *c;
    }
    out
}

#[derive(Default)]
pub struct CameraChain {
    pub gi: usize,
    pub lp_array: usize,
    pub local_player: usize,
    pub pc: usize,
    pub cm: usize,
    pub pov_offset: usize,
    pub location: [f32; 3],
    pub rotation: [f32; 3],
    pub fov: f32,
    pub ok: bool,
    pub candidates: [PovCandidate; POV_CANDIDATE_COUNT],
}

pub fn get_camera_chain(world: usize, forced_pov: usize) -> CameraChain {
    let mut c = CameraChain::default();
    if world == 0 { return c; }

    c.gi = safe_read_ptr(world + GAME_INSTANCE_OFFSET);
    if c.gi == 0 { return c; }

    c.lp_array = safe_read_ptr(c.gi + LOCAL_PLAYERS_OFFSET);
    if c.lp_array == 0 { return c; }

    c.local_player = safe_read_ptr(c.lp_array);
    if c.local_player == 0 { return c; }

    c.pc = safe_read_ptr(c.local_player + PLAYER_CONTROLLER_OFFSET);
    if c.pc == 0 { return c; }

    c.cm = safe_read_ptr(c.pc + CAMERA_MANAGER_OFFSET);
    if c.cm == 0 { return c; }

    c.candidates = scan_pov_candidates(c.cm);

    let pov_off = if forced_pov != 0 {
        forced_pov
    } else if c.candidates[0].offset != 0 {
        c.candidates[0].offset
    } else {
        return c;
    };
    c.pov_offset = pov_off;

    let loc = match safe_read_vec3(c.cm + pov_off) {
        Some(v) => v,
        None => return c,
    };
    let rot = match safe_read_vec3(c.cm + pov_off + 0xC) {
        Some(v) => v,
        None => return c,
    };
    let fov = match safe_read_f32(c.cm + pov_off + 0x18) {
        Some(v) => v,
        None => return c,
    };
    if fov < 1.0 || fov > 179.0 {
        return c;
    }

    c.location = loc;
    c.rotation = rot;
    c.fov = fov;
    c.ok = true;
    c
}
