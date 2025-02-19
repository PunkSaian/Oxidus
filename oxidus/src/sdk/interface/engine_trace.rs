use std::{
    alloc::{alloc, Layout},
    mem::MaybeUninit,
};

use macros::vmt;

use crate::{
    math::Vector3,
    prelude::Interfaces,
    sdk::{
        bindings::{BaseEntity, TFPlayer},
        class_id::ClassId,
    },
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Plane {
    normal: Vector3,
    dist: f32,
    r#type: u8,
    signbits: u8,
    pad: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Surface {
    name: *const u8,
    surface_props: i16,
    flags: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Trace {
    pub startpos: Vector3,
    pub endpos: Vector3,
    pub plane: Plane,
    pub fraction: f32,
    pub contents: i32,
    pub disp_flags: u16,
    pub allsolid: bool,
    pub startsolid: bool,
    pub fraction_left_solid: f32,
    pub surface: Surface,
    pub hit_group: i32,
    pub physics_bone: i16,
    pub entity: *const BaseEntity,
    pub hitbox_id: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct TraceFilter {
    vmt: *mut TraceFilterVMT,
    owner: &'static TFPlayer,
}

impl std::fmt::Debug for TraceFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //print vmt
        f.debug_struct("TraceFilter")
            .field("vmt", &self.vmt)
            .finish()
    }
}

#[repr(C)]
pub enum TraceType {
    Everything = 0,
    WorldOnly,
    EntitiesOnly,
    EverythingFilterProps,
}

#[repr(C)]
pub struct TraceFilterVMT {
    should_hit_entity:
        extern "C" fn(&'static TraceFilter, &'static BaseEntity, content_mask: i32) -> bool,
    get_trace_type: extern "C" fn(&'static TraceFilter) -> TraceType,
}

extern "C" fn should_hit_entity(
    trace_filter: &'static TraceFilter,
    ent: &'static BaseEntity,
    _: i32,
) -> bool {
    if ent.get_entindex() == trace_filter.owner.get_entindex() {
        return false;
    }
    let networkable = ent.as_networkable();
    let class = networkable.get_client_class();
    if class.class_id == ClassId::CFuncRespawnRoomVisualizer {
        return false;
    }
    true
}

extern "C" fn get_trace_type(_: &'static TraceFilter) -> TraceType {
    TraceType::Everything
}

impl TraceFilter {
    pub fn new(p_local: &'static TFPlayer) -> TraceFilter {
        let vmt = unsafe { alloc(Layout::new::<TraceFilterVMT>()).cast::<TraceFilterVMT>() };

        unsafe {
            *vmt = TraceFilterVMT {
                should_hit_entity,
                get_trace_type,
            };
        }

        TraceFilter {
            vmt,
            owner: p_local,
        }
    }
}

pub struct EngineTrace;

#[vmt]
pub struct EngineTrace {
    #[offset(4)]
    pub trace_ray:
        extern "C" fn(ray: &Ray, mask: u32, trace_filter: &TraceFilter, trace: &mut Trace) -> i32,
}

impl EngineTrace {
    pub fn trace(&self, owner: &'static TFPlayer, start: Vector3, end: Vector3, mask: u32) -> Trace {

        let ray = Ray::new(start, end);
        let filter = TraceFilter::new(owner);
        let mut trace = unsafe { MaybeUninit::zeroed().assume_init() };

        self.trace_ray(&ray, mask, &filter, &mut trace);
        trace
    }
}

#[repr(C, align(16))]
#[derive(Debug, Clone)]
pub struct VectorAligned {
    vec: Vector3,
    _pad: i32,
}
impl VectorAligned {
    pub fn new(vec: Vector3) -> Self {
        VectorAligned { vec, _pad: 0 }
    }
}
impl Default for VectorAligned {
    fn default() -> Self {
        VectorAligned::new(Vector3::empty())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ray {
    pub start: VectorAligned,
    pub delta: VectorAligned,
    pub start_offset: VectorAligned,
    pub extents: VectorAligned,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    pub fn new(start: Vector3, end: Vector3) -> Self {
        let delta = end - start;
        Ray {
            start: VectorAligned::new(start),
            delta: VectorAligned::new(delta),
            start_offset: VectorAligned::default(),
            extents: VectorAligned::default(),
            is_ray: true,
            is_swept: delta.len() != 0f32,
        }
    }
}

pub const CONTENTS_EMPTY: u32 = 0x0;
pub const CONTENTS_SOLID: u32 = 0x1;
pub const CONTENTS_WINDOW: u32 = 0x2;
pub const CONTENTS_AUX: u32 = 0x4;
pub const CONTENTS_GRATE: u32 = 0x8;
pub const CONTENTS_SLIME: u32 = 0x10;
pub const CONTENTS_WATER: u32 = 0x20;
pub const CONTENTS_BLOCKLOS: u32 = 0x40;
pub const CONTENTS_OPAQUE: u32 = 0x80;
pub const LAST_VISIBLE_CONTENTS: u32 = 0x80;
pub const ALL_VISIBLE_CONTENTS: u32 = LAST_VISIBLE_CONTENTS | (LAST_VISIBLE_CONTENTS - 1);
pub const CONTENTS_TESTFOGVOLUME: u32 = 0x100;
pub const CONTENTS_UNUSED: u32 = 0x200;
pub const CONTENTS_UNUSED6: u32 = 0x400;
pub const CONTENTS_TEAM1: u32 = 0x800;
pub const CONTENTS_TEAM2: u32 = 0x1000;
pub const CONTENTS_IGNORE_NODRAW_OPAQUE: u32 = 0x2000;
pub const CONTENTS_MOVEABLE: u32 = 0x4000;
pub const CONTENTS_AREAPORTAL: u32 = 0x8000;
pub const CONTENTS_PLAYERCLIP: u32 = 0x10000;
pub const CONTENTS_MONSTERCLIP: u32 = 0x20000;
pub const CONTENTS_CURRENT_0: u32 = 0x40000;
pub const CONTENTS_CURRENT_90: u32 = 0x80000;
pub const CONTENTS_CURRENT_180: u32 = 0x0010_0000;
pub const CONTENTS_CURRENT_270: u32 = 0x0020_0000;
pub const CONTENTS_CURRENT_UP: u32 = 0x0040_0000;
pub const CONTENTS_CURRENT_DOWN: u32 = 0x0080_0000;
pub const CONTENTS_ORIGIN: u32 = 0x0100_0000;
pub const CONTENTS_MONSTER: u32 = 0x0200_0000;
pub const CONTENTS_DEBRIS: u32 = 0x0400_0000;
pub const CONTENTS_DETAIL: u32 = 0x0800_0000;
pub const CONTENTS_TRANSLUCENT: u32 = 0x1000_0000;
pub const CONTENTS_LADDER: u32 = 0x2000_0000;
pub const CONTENTS_HITBOX: u32 = 0x4000_0000;

pub const MASK_ALL: u32 = 0xFFFF_FFFF;
pub const MASK_SOLID: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTER | CONTENTS_GRATE;
pub const MASK_PLAYERSOLID: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_PLAYERCLIP
    | CONTENTS_WINDOW
    | CONTENTS_MONSTER
    | CONTENTS_GRATE;
pub const MASK_NPCSOLID: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTERCLIP
    | CONTENTS_WINDOW
    | CONTENTS_MONSTER
    | CONTENTS_GRATE;
pub const MASK_WATER: u32 = CONTENTS_WATER | CONTENTS_MOVEABLE | CONTENTS_SLIME;
pub const MASK_OPAQUE: u32 = CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_OPAQUE;
pub const MASK_OPAQUE_AND_NPCS: u32 = MASK_OPAQUE | CONTENTS_MONSTER;
pub const MASK_BLOCKLOS: u32 = CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_BLOCKLOS;
pub const MASK_BLOCKLOS_AND_NPCS: u32 = MASK_BLOCKLOS | CONTENTS_MONSTER;
pub const MASK_VISIBLE: u32 = MASK_OPAQUE | CONTENTS_IGNORE_NODRAW_OPAQUE;
pub const MASK_VISIBLE_AND_NPCS: u32 = MASK_OPAQUE_AND_NPCS | CONTENTS_IGNORE_NODRAW_OPAQUE;
pub const MASK_SHOT: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTER
    | CONTENTS_WINDOW
    | CONTENTS_DEBRIS
    | CONTENTS_HITBOX;
pub const MASK_SHOT_HULL: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTER
    | CONTENTS_WINDOW
    | CONTENTS_DEBRIS
    | CONTENTS_GRATE;
pub const MASK_SHOT_PORTAL: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTER;
pub const MASK_SOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_GRATE;
pub const MASK_PLAYERSOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_PLAYERCLIP | CONTENTS_GRATE;
pub const MASK_NPCSOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTERCLIP | CONTENTS_GRATE;
pub const MASK_NPCWORLDSTATIC: u32 =
    CONTENTS_SOLID | CONTENTS_WINDOW | CONTENTS_MONSTERCLIP | CONTENTS_GRATE;
pub const MASK_SPLITAREAPORTAL: u32 = CONTENTS_WATER | CONTENTS_SLIME;
pub const MASK_CURRENT: u32 = CONTENTS_CURRENT_0
    | CONTENTS_CURRENT_90
    | CONTENTS_CURRENT_180
    | CONTENTS_CURRENT_270
    | CONTENTS_CURRENT_UP
    | CONTENTS_CURRENT_DOWN;
pub const MASK_DEADSOLID: u32 =
    CONTENTS_SOLID | CONTENTS_PLAYERCLIP | CONTENTS_WINDOW | CONTENTS_GRATE;
