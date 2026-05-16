//! Universal UE4 class inheritance filter.
//!
//! Reusable across UE4 (4.27-era) games: caller supplies the module base and a
//! `GObjects` offset and gets back resolved `UClass*` pointers by name. Subsequent
//! filtering is pure pointer comparison along the `UStruct::SuperStruct` chain,
//! no FName-to-string per actor.
//!
//! To reuse in another project: swap `memory::GOBJECTS_OFFSET` and pass any base
//! class names — `find_class_by_name` walks GObjects, `ClassFilter::classify`
//! walks SuperStruct.

#![allow(dead_code)] // public API intended for reuse — some helpers unused in this binary

use std::collections::HashMap;
use crate::memory;

// UStruct layout (UE4.27, stable across builds)
pub const USTRUCT_SUPER_OFFSET: usize = 0x40;

// For Greylock-Win64-Shipping, Dumper-7 reports GObjects pointing DIRECTLY at
// TUObjectArray — there is no outer FUObjectArray wrapper. Layout from the dumped
// SDK's CppSDK/SDK/Basic.hpp:
//   +0x00  FUObjectItem**  Objects       (chunk-pointer array)
//   +0x08  pad[0x8]                       (no PreAllocatedObjects field)
//   +0x10  int32           MaxElements
//   +0x14  int32           NumElements
//   +0x18  int32           MaxChunks
//   +0x1C  int32           NumChunks
const OBJ_OBJECTS_OFFSET: usize = 0x00;
const NUM_ELEMENTS_OFFSET: usize = 0x14;

// FUObjectItem (size 0x18): UObject* Object at +0x00, then 0x10 bytes of flags/cluster/serial
const ITEM_SIZE: usize = 0x18;
const CHUNK_ENTRIES: usize = 0x10000;

const MAX_OBJECTS_SANITY_CAP: i32 = 4_000_000;
const MAX_SUPER_DEPTH: u32 = 32;

/// Reads i32 values at GObjects + 0x00, 0x04, ..., 0x1C so the Debug tab can
/// show the raw fields of TUObjectArray. The two count fields (MaxElements,
/// NumElements) live at +0x10 and +0x14.
pub fn probe_gobjects_layout(module_base: usize, gobjects_offset: usize) -> [i32; 8] {
    let mut out = [0i32; 8];
    if module_base == 0 {
        return out;
    }
    let gobjects = module_base + gobjects_offset;
    for (i, slot) in out.iter_mut().enumerate() {
        let off = i * 4;
        *slot = memory::read_i32_at(gobjects + off).unwrap_or(0);
    }
    out
}

#[derive(Default, Clone, Copy)]
pub struct GObjectsStats {
    pub gobjects_addr: usize,
    pub chunks_array: usize,
    pub num_elements: i32,
    pub num_elements_offset: usize,
    pub visited: i32,
}

pub fn walk_gobjects<F: FnMut(usize)>(
    module_base: usize,
    gobjects_offset: usize,
    mut visit: F,
) -> GObjectsStats {
    let mut stats = GObjectsStats::default();
    if module_base == 0 {
        return stats;
    }
    let gobjects = module_base + gobjects_offset;
    stats.gobjects_addr = gobjects;

    let chunks_array = memory::read_ptr_at(gobjects + OBJ_OBJECTS_OFFSET);
    stats.chunks_array = chunks_array;
    if chunks_array == 0 {
        return stats;
    }
    let num_elements = memory::read_i32_at(gobjects + NUM_ELEMENTS_OFFSET).unwrap_or(0);
    stats.num_elements = num_elements;
    stats.num_elements_offset = NUM_ELEMENTS_OFFSET;
    if num_elements <= 0 || num_elements > MAX_OBJECTS_SANITY_CAP {
        return stats;
    }

    let total = num_elements as usize;
    let mut last_chunk_idx = usize::MAX;
    let mut last_chunk_ptr = 0usize;
    for i in 0..total {
        let chunk_idx = i / CHUNK_ENTRIES;
        let in_chunk = i % CHUNK_ENTRIES;
        let chunk_ptr = if chunk_idx == last_chunk_idx {
            last_chunk_ptr
        } else {
            let p = memory::read_ptr_at(chunks_array + chunk_idx * 8);
            last_chunk_idx = chunk_idx;
            last_chunk_ptr = p;
            p
        };
        if chunk_ptr == 0 {
            continue;
        }
        let item_addr = chunk_ptr + in_chunk * ITEM_SIZE;
        let obj_ptr = memory::read_ptr_at(item_addr);
        if obj_ptr == 0 {
            continue;
        }
        stats.visited += 1;
        visit(obj_ptr);
    }
    stats
}

pub fn find_class_by_name(
    module_base: usize,
    gobjects_offset: usize,
    target_name: &str,
) -> Option<usize> {
    let (results, _) = find_classes_by_names(module_base, gobjects_offset, &[target_name]);
    results.into_iter().next().flatten()
}

/// Single-pass GObjects walk that resolves multiple class pointers by name.
/// Returns (results in same order as `target_names`, walk stats).
pub fn find_classes_by_names(
    module_base: usize,
    gobjects_offset: usize,
    target_names: &[&str],
) -> (Vec<Option<usize>>, GObjectsStats) {
    let mut results: Vec<Option<usize>> = vec![None; target_names.len()];
    let mut remaining = target_names.len();
    let stats = walk_gobjects(module_base, gobjects_offset, |obj_ptr| {
        if remaining == 0 {
            return;
        }
        let name = match memory::resolve_fname(module_base, obj_ptr + memory::UOBJECT_NAME_OFFSET) {
            Some(n) => n,
            None => return,
        };
        for (i, target) in target_names.iter().enumerate() {
            if results[i].is_none() && name == *target {
                results[i] = Some(obj_ptr);
                remaining -= 1;
                break;
            }
        }
    });
    (results, stats)
}

pub fn is_subclass_of(
    class_ptr: usize,
    base: usize,
    cache: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if class_ptr == 0 || base == 0 {
        return false;
    }
    let key = (class_ptr, base);
    if let Some(&v) = cache.get(&key) {
        return v;
    }
    let mut cur = class_ptr;
    let mut depth = 0u32;
    let result = loop {
        if cur == base {
            break true;
        }
        if cur == 0 || depth >= MAX_SUPER_DEPTH {
            break false;
        }
        cur = memory::read_ptr_at(cur + USTRUCT_SUPER_OFFSET);
        depth += 1;
    };
    cache.insert(key, result);
    result
}

/// Pointer-comparison inheritance filter. `roots` is an ordered list of base
/// class pointers; `classify` returns the first root encountered while walking
/// the class's SuperStruct chain (so list more-derived roots first if you want
/// to distinguish e.g. `HumanPlayer` from `Human`).
pub struct ClassFilter {
    pub roots: Vec<usize>,
    cache: HashMap<usize, Option<usize>>,
}

impl ClassFilter {
    pub fn new(roots: Vec<usize>) -> Self {
        Self {
            roots: roots.into_iter().filter(|&p| p != 0).collect(),
            cache: HashMap::new(),
        }
    }

    pub fn classify(&mut self, class_ptr: usize) -> Option<usize> {
        if class_ptr == 0 || self.roots.is_empty() {
            return None;
        }
        if let Some(&v) = self.cache.get(&class_ptr) {
            return v;
        }
        let mut cur = class_ptr;
        let mut depth = 0u32;
        let result = loop {
            if cur == 0 || depth >= MAX_SUPER_DEPTH {
                break None;
            }
            if let Some(&hit) = self.roots.iter().find(|&&r| r == cur) {
                break Some(hit);
            }
            cur = memory::read_ptr_at(cur + USTRUCT_SUPER_OFFSET);
            depth += 1;
        };
        self.cache.insert(class_ptr, result);
        result
    }

    pub fn matches(&mut self, class_ptr: usize) -> bool {
        self.classify(class_ptr).is_some()
    }
}
