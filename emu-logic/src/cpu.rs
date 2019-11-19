use crate::{cartridge_manifest::CartridgeManifest, fs_module_loader::FsModuleReader};
use kaiju_core::{error::*, program::compile_ops_descriptor, validator::EmptyDeepValidator};
use kaiju_vm_core::{
    load_cstring,
    processor::{OpAction, Processor},
    state::Value,
    vm::Vm,
};
use rand::random;
use std::{ffi::CString, fs::read_to_string, path::Path, sync::Mutex};

const OPSDESC: &str = include_str!("./opsdesc.txt");
const SPRCOUNT: i16 = 128;
const STACKSIZE: usize = 1024;
const MEMSIZE: usize = 8 * 1024;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::default());
    static ref CPU_INSTANCE: Mutex<CPU> = Mutex::new(CPU::default());
}

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub image: i16,
    pub x: i16,
    pub y: i16,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            image: -1,
            x: 0,
            y: 0,
        }
    }
}

impl Sprite {
    pub fn collide(&self, other: &Self, manifest: &CartridgeManifest) -> bool {
        if self.image < 0 || other.image < 0 {
            return false;
        }
        let img_a = &manifest.images[self.image as usize];
        let img_b = &manifest.images[other.image as usize];
        let axf = self.x - img_a.ox as i16;
        let axt = axf + img_a.w as i16;
        let ayf = self.y - img_a.oy as i16;
        let ayt = ayf + img_a.h as i16;
        let bxf = other.x - img_b.ox as i16;
        let bxt = bxf + img_b.w as i16;
        let byf = other.y - img_b.oy as i16;
        let byt = byf + img_b.h as i16;
        axt > bxf && axf < bxt && ayt > byf && ayf < byt
    }
}

#[derive(Default)]
pub struct State {
    pub cartridge_manifest: Option<CartridgeManifest>,
    pub halt: bool,
    pub input: i16,
    pub bgcolor: u32,
    pub viewport_x: i16,
    pub viewport_y: i16,
    pub sprites: Vec<Sprite>,
    pub cached_string: CString,
    pub cached_errors: String,
}

impl State {
    pub fn start(&mut self, manifest: CartridgeManifest) {
        self.stop();
        self.cartridge_manifest = Some(manifest);
        self.halt = false;
        self.input = 0;
        self.viewport_x = 0;
        self.viewport_y = 0;
        self.sprites = std::iter::repeat(Sprite::default())
            .take(SPRCOUNT as usize)
            .collect();
    }

    pub fn stop(&mut self) {
        self.cartridge_manifest = None;
        self.sprites.clear();
    }

    pub fn with<T, F>(mut cb: F) -> T
    where
        F: FnMut(&Self) -> T,
    {
        cb(&STATE.lock().unwrap())
    }

    pub fn with_mut<T, F>(mut cb: F) -> T
    where
        F: FnMut(&mut Self) -> T,
    {
        cb(&mut STATE.lock().unwrap())
    }
}

#[derive(Default)]
pub struct CPU {
    pub vm: Option<Vm>,
}

impl CPU {
    pub fn start(&mut self, rom_path: &Path) -> SimpleResult<()> {
        self.stop();
        let manifest_path = rom_path.join("manifest.json");
        let manifest_content = match read_to_string(&manifest_path) {
            Ok(content) => content,
            Err(error) => {
                return Err(SimpleError::new(format!(
                    "Could not read manifest ({:?}): {}",
                    manifest_path, error
                )))
            }
        };
        let manifest = match serde_json::from_str::<CartridgeManifest>(&manifest_content) {
            Ok(manifest) => manifest,
            Err(error) => {
                return Err(SimpleError::new(format!(
                    "Manifest parsing error: {}",
                    error
                )))
            }
        };
        let entry_path = if let Some(path) = rom_path.join(&manifest.code_entry_path).to_str() {
            path.to_owned()
        } else {
            return Err(SimpleError::new(format!(
                "Invalid path format: {:?}",
                manifest.code_entry_path,
            )));
        };
        let opsdesc = match compile_ops_descriptor(OPSDESC) {
            Ok(opsdesc) => opsdesc,
            Err(error) => return Err(SimpleError::new(error.pretty)),
        };
        State::with_mut(|c| c.start(manifest.clone()));
        self.vm = Some(Vm::from_source::<EmptyDeepValidator, _>(
            &entry_path,
            FsModuleReader::default(),
            &opsdesc,
            STACKSIZE,
            MEMSIZE,
        )?);
        self.vm.as_mut().unwrap().start("main")?;
        Ok(())
    }

    pub fn stop(&mut self) {
        State::with_mut(|c| c.stop());
        self.vm = None;
    }

    pub fn with<T, F>(mut cb: F) -> T
    where
        F: FnMut(&Self) -> T,
    {
        cb(&CPU_INSTANCE.lock().unwrap())
    }

    pub fn with_mut<T, F>(mut cb: F) -> T
    where
        F: FnMut(&mut Self) -> T,
    {
        cb(&mut CPU_INSTANCE.lock().unwrap())
    }
}

impl Processor for CPU {
    fn process_op(
        op: &String,
        params: &[usize],
        targets: &[usize],
        vm: &mut Vm,
    ) -> SimpleResult<OpAction> {
        match op.as_str() {
            "halt" => {
                State::with_mut(|p| p.halt = true);
                Ok(OpAction::None)
            }
            "goto" => {
                let v = load_cstring(params[0], vm)?;
                if let Some(pos) = vm.find_label(&v) {
                    Ok(OpAction::GoTo(pos))
                } else {
                    Err(SimpleError::new(format!(
                        "Function does not have `{}` label",
                        v
                    )))
                }
            }
            "if" => {
                let v = vm.state().load_data::<i16>(params[0])?;
                let th = load_cstring(params[1], vm)?;
                let th = if let Some(pos) = vm.find_label(&th) {
                    pos
                } else {
                    return Err(SimpleError::new(format!(
                        "Function does not have `{}` label",
                        v
                    )));
                };
                let el = load_cstring(params[2], vm)?;
                let el = if let Some(pos) = vm.find_label(&el) {
                    pos
                } else {
                    return Err(SimpleError::new(format!(
                        "Function does not have `{}` label",
                        v
                    )));
                };
                Ok(OpAction::GoTo(if v != 0 { th } else { el }))
            }
            "ret" => Ok(OpAction::Return),
            "pass" => Ok(OpAction::None),
            "dbgi" => {
                let v = vm.state().load_data::<i16>(params[0])?;
                State::with_mut(|c| {
                    c.cached_errors = if c.cached_errors.is_empty() {
                        format!("{}", v)
                    } else {
                        format!("{}\n{}", c.cached_errors, v)
                    };
                });
                Ok(OpAction::None)
            }
            "dbgs" => {
                let v = load_cstring(params[0], vm)?;
                State::with_mut(|c| {
                    c.cached_errors = if c.cached_errors.is_empty() {
                        format!("{}", v)
                    } else {
                        format!("{}\n{}", c.cached_errors, v)
                    };
                });
                Ok(OpAction::None)
            }
            "dbgp" => {
                let p = vm.state().load_data::<usize>(params[0])?;
                State::with_mut(|c| {
                    c.cached_errors = if c.cached_errors.is_empty() {
                        format!("{:#X} ({})", p, p)
                    } else {
                        format!("{}\n{:#X} ({})", c.cached_errors, p, p)
                    };
                });
                Ok(OpAction::None)
            }
            "dbgm" => {
                let a = vm.state().load_data::<usize>(params[0])?;
                let s = vm.state().load_data::<i16>(params[1])? as usize;
                let v = vm.state().load_bytes(a, s)?;
                State::with_mut(|c| {
                    c.cached_errors = if c.cached_errors.is_empty() {
                        format!("{:?}", v)
                    } else {
                        format!("{}\n{:?}", c.cached_errors, v)
                    };
                });
                Ok(OpAction::None)
            }
            "cstp" => {
                let a = vm.state().load_data::<usize>(params[0])?;
                vm.state_mut().store_data(targets[0], &a)?;
                Ok(OpAction::None)
            }
            "poff" => {
                let a = vm.state().load_data::<usize>(params[0])?;
                let o = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], &((a as isize + o as isize) as usize))?;
                Ok(OpAction::None)
            }
            "i2b" => {
                let v = vm.state().load_data::<i16>(params[0])?;
                vm.state_mut().store_data(targets[0], &(v as u8))?;
                Ok(OpAction::None)
            }
            "b2i" => {
                let v = vm.state().load_data::<u8>(params[0])?;
                vm.state_mut().store_data(targets[0], &i16::from(v))?;
                Ok(OpAction::None)
            }
            "allc" => {
                let size = vm.state().load_data::<i16>(params[0])? as usize;
                let v = vm.state_mut().alloc_memory_value(size + 2)?;
                vm.state_mut().store_data(v.address, &(v.size as u16))?;
                vm.state_mut().store_data(targets[0], &(v.address + 2))?;
                Ok(OpAction::None)
            }
            "free" => {
                let address = vm.state().load_data::<usize>(params[0])?;
                let size = vm.state().load_data::<u16>(address - 2)? + 2;
                vm.state_mut()
                    .dealloc_memory_value(&Value::new(address - 2, size as usize))?;
                Ok(OpAction::None)
            }
            "inp" => {
                let input = State::with_mut(|p| p.input);
                vm.state_mut().store_data(targets[0], &input)?;
                Ok(OpAction::None)
            }
            "bgc" => {
                let r = vm.state().load_data::<i16>(params[0])?;
                let g = vm.state().load_data::<i16>(params[1])?;
                let b = vm.state().load_data::<i16>(params[2])?;
                let rgb =
                    (((r as u32) & 0xff) << 16) | (((g as u32) & 0xff) << 8) | (b as u32) & 0xff;
                State::with_mut(|c| c.bgcolor = rgb);
                Ok(OpAction::None)
            }
            "svp" => {
                let x = vm.state().load_data::<i16>(params[0])?;
                let y = vm.state().load_data::<i16>(params[1])?;
                State::with_mut(|c| {
                    c.viewport_x = x;
                    c.viewport_y = y;
                });
                Ok(OpAction::None)
            }
            "gvp" => {
                let (x, y) = State::with(|c| (c.viewport_x, c.viewport_y));
                vm.state_mut().store_data(targets[0], &x)?;
                vm.state_mut().store_data(targets[1], &y)?;
                Ok(OpAction::None)
            }
            "ssi" => {
                let idx = vm.state().load_data::<i16>(params[0])?;
                let idx = if idx >= 0 && idx < SPRCOUNT {
                    idx as usize
                } else {
                    return Err(SimpleError::new(format!(
                        "Sprite index out of bounds: {}",
                        idx
                    )));
                };
                let img = vm.state().load_data::<i16>(params[1])?;
                State::with_mut(|c| c.sprites[idx].image = img);
                Ok(OpAction::None)
            }
            "gsi" => {
                let idx = vm.state().load_data::<i16>(params[0])?;
                let idx = if idx >= 0 && idx < SPRCOUNT {
                    idx as usize
                } else {
                    return Err(SimpleError::new(format!(
                        "Sprite index out of bounds: {}",
                        idx
                    )));
                };
                let img = State::with(|c| c.sprites[idx].image);
                vm.state_mut().store_data(targets[0], &img)?;
                Ok(OpAction::None)
            }
            "ssp" => {
                let idx = vm.state().load_data::<i16>(params[0])?;
                let idx = if idx >= 0 && idx < SPRCOUNT {
                    idx as usize
                } else {
                    return Err(SimpleError::new(format!(
                        "Sprite index out of bounds: {}",
                        idx
                    )));
                };
                let x = vm.state().load_data::<i16>(params[1])?;
                let y = vm.state().load_data::<i16>(params[2])?;
                State::with_mut(|c| {
                    c.sprites[idx].x = x;
                    c.sprites[idx].y = y;
                });
                Ok(OpAction::None)
            }
            "gsp" => {
                let idx = vm.state().load_data::<i16>(params[0])?;
                let idx = if idx >= 0 && idx < SPRCOUNT {
                    idx as usize
                } else {
                    return Err(SimpleError::new(format!(
                        "Sprite index out of bounds: {}",
                        idx
                    )));
                };
                let (x, y) = State::with(|c| (c.sprites[idx].x, c.sprites[idx].y));
                vm.state_mut().store_data(targets[0], &x)?;
                vm.state_mut().store_data(targets[1], &y)?;
                Ok(OpAction::None)
            }
            "coll" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                let a = if a >= 0 && a < SPRCOUNT {
                    a as usize
                } else {
                    return Err(SimpleError::new(format!(
                        "Sprite index out of bounds: {}",
                        a
                    )));
                };
                let b = if b >= 0 && b < SPRCOUNT {
                    b as usize
                } else {
                    return Err(SimpleError::new(format!(
                        "Sprite index out of bounds: {}",
                        b
                    )));
                };
                vm.state_mut().store_data(
                    targets[0],
                    &State::with(|c| {
                        (c.sprites[a]
                            .collide(&c.sprites[b], c.cartridge_manifest.as_ref().unwrap()))
                    }),
                )?;
                Ok(OpAction::None)
            }
            "bor" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], &(if a != 0 || b != 0 { 1 } else { 0 }))?;
                Ok(OpAction::None)
            }
            "band" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], &(if a != 0 && b != 0 { 1 } else { 0 }))?;
                Ok(OpAction::None)
            }
            "inc" => {
                let v = vm.state().load_data::<i16>(params[0])?;
                vm.state_mut().store_data(targets[0], &(v + 1))?;
                Ok(OpAction::None)
            }
            "dec" => {
                let v = vm.state().load_data::<i16>(params[0])?;
                vm.state_mut().store_data(targets[0], &(v - 1))?;
                Ok(OpAction::None)
            }
            "add" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a + b))?;
                Ok(OpAction::None)
            }
            "sub" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a - b))?;
                Ok(OpAction::None)
            }
            "mul" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a * b))?;
                Ok(OpAction::None)
            }
            "div" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a / b))?;
                Ok(OpAction::None)
            }
            "mod" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a % b))?;
                Ok(OpAction::None)
            }
            "mov" => {
                let v = vm.state().load_data::<i16>(params[0])?;
                vm.state_mut().store_data(targets[0], &v)?;
                Ok(OpAction::None)
            }
            "eq" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], if a == b { &1i16 } else { &0i16 })?;
                Ok(OpAction::None)
            }
            "ne" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], if a != b { &1i16 } else { &0i16 })?;
                Ok(OpAction::None)
            }
            "gt" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], if a > b { &1i16 } else { &0i16 })?;
                Ok(OpAction::None)
            }
            "lt" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], if a < b { &1i16 } else { &0i16 })?;
                Ok(OpAction::None)
            }
            "ge" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], if a >= b { &1i16 } else { &0i16 })?;
                Ok(OpAction::None)
            }
            "le" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut()
                    .store_data(targets[0], if a <= b { &1i16 } else { &0i16 })?;
                Ok(OpAction::None)
            }
            "lsh" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a << b))?;
                Ok(OpAction::None)
            }
            "rsh" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a >> b))?;
                Ok(OpAction::None)
            }
            "and" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a & b))?;
                Ok(OpAction::None)
            }
            "or" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a | b))?;
                Ok(OpAction::None)
            }
            "xor" => {
                let a = vm.state().load_data::<i16>(params[0])?;
                let b = vm.state().load_data::<i16>(params[1])?;
                vm.state_mut().store_data(targets[0], &(a ^ b))?;
                Ok(OpAction::None)
            }
            "neg" => {
                let v = vm.state().load_data::<i16>(params[0])?;
                vm.state_mut().store_data(targets[0], &(!v))?;
                Ok(OpAction::None)
            }
            "rnd" => {
                vm.state_mut().store_data::<i16>(targets[0], &random())?;
                Ok(OpAction::None)
            }
            _ => Err(SimpleError::new(format!("Unsupported op: {}", op))),
        }
    }
}
