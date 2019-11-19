#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod cartridge_manifest;
mod cpu;
mod fs_module_loader;

use crate::cpu::{State, CPU};
use std::{ffi::CString, path::PathBuf};

type GmlString = *const libc::c_char;
type GmlNumber = libc::c_double;

const GML_FALSE: GmlNumber = 0.0;
const GML_TRUE: GmlNumber = 1.0;

#[no_mangle]
pub extern "C" fn emu_last_errors() -> GmlString {
    State::with_mut(|c| {
        c.cached_string = CString::new(c.cached_errors.as_str()).unwrap();
        c.cached_errors.clear();
        c.cached_string.as_ptr() as GmlString
    })
}

#[no_mangle]
pub extern "C" fn emu_cpu_start(rom_path: GmlString) -> GmlNumber {
    if rom_path.is_null() {
        return GML_FALSE;
    }
    let rom_path: PathBuf = string_from_raw_unsized(rom_path as *const libc::c_uchar).into();
    CPU::with_mut(|c| {
        if let Err(err) = c.start(&rom_path) {
            State::with_mut(|c| {
                c.cached_errors = if c.cached_errors.is_empty() {
                    err.message.clone()
                } else {
                    format!("{}\n{}", c.cached_errors, err.message)
                };
            });
            GML_FALSE
        } else {
            GML_TRUE
        }
    })
}

#[no_mangle]
pub extern "C" fn emu_cpu_stop() -> GmlNumber {
    CPU::with_mut(|c| c.stop());
    GML_TRUE
}

#[no_mangle]
pub extern "C" fn emu_cpu_resume() -> GmlNumber {
    CPU::with_mut(|c| {
        if let Some(vm) = &mut c.vm {
            if vm.can_resume() && vm.resume::<CPU>().is_ok() {
                return GML_TRUE;
            }
        }
        GML_FALSE
    })
}

#[no_mangle]
pub extern "C" fn emu_cpu_can_resume() -> GmlNumber {
    CPU::with(|c| {
        if let Some(vm) = &c.vm {
            if vm.can_resume() {
                return GML_TRUE;
            }
        }
        GML_FALSE
    })
}

#[no_mangle]
pub extern "C" fn emu_cpu_is_halt() -> GmlNumber {
    State::with(|c| if c.halt { GML_TRUE } else { GML_FALSE })
}

#[no_mangle]
pub extern "C" fn emu_cpu_clear_halt() -> GmlNumber {
    State::with_mut(|c| c.halt = false);
    GML_TRUE
}

#[no_mangle]
pub extern "C" fn emu_set_input(flags: GmlNumber) -> GmlNumber {
    State::with_mut(|c| c.input = flags as i16);
    GML_TRUE
}

#[no_mangle]
pub extern "C" fn emu_bgcolor() -> GmlNumber {
    State::with(|c| c.bgcolor as GmlNumber)
}

#[no_mangle]
pub extern "C" fn emu_viewport_x() -> GmlNumber {
    State::with(|c| c.viewport_x as GmlNumber)
}

#[no_mangle]
pub extern "C" fn emu_viewport_y() -> GmlNumber {
    State::with(|c| c.viewport_y as GmlNumber)
}

#[no_mangle]
pub extern "C" fn emu_sprites_count() -> GmlNumber {
    State::with(|c| c.sprites.len() as GmlNumber)
}

#[no_mangle]
pub extern "C" fn emu_sprite_image(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if index < 0.0 {
            return -1.0;
        }
        let index = index as usize;
        if index >= c.sprites.len() {
            return -1.0;
        }
        c.sprites[index].image as GmlNumber
    })
}

#[no_mangle]
pub extern "C" fn emu_sprite_x(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if index < 0.0 {
            return 0.0;
        }
        let index = index as usize;
        if index >= c.sprites.len() {
            return 0.0;
        }
        c.sprites[index].x as GmlNumber
    })
}

#[no_mangle]
pub extern "C" fn emu_sprite_y(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if index < 0.0 {
            return 0.0;
        }
        let index = index as usize;
        if index >= c.sprites.len() {
            return 0.0;
        }
        c.sprites[index].y as GmlNumber
    })
}

#[no_mangle]
pub extern "C" fn emu_manifest_images_atlas_path() -> GmlString {
    State::with_mut(|c| {
        if let Some(manifest) = &c.cartridge_manifest {
            if let Some(path) = manifest.images_atlas_path.to_str() {
                c.cached_string = CString::new(path).unwrap();
            } else {
                c.cached_string = CString::new("").unwrap();
            }
        } else {
            c.cached_string = CString::new("").unwrap();
        }
        c.cached_string.as_ptr() as GmlString
    })
}

#[no_mangle]
pub extern "C" fn emu_manifest_images_count() -> GmlNumber {
    State::with(|c| {
        if let Some(manifest) = &c.cartridge_manifest {
            manifest.images.len() as GmlNumber
        } else {
            -1.0
        }
    })
}

#[no_mangle]
pub extern "C" fn emu_manifest_image_x(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if let Some(manifest) = &c.cartridge_manifest {
            manifest.images[index as usize].x as GmlNumber
        } else {
            0.0
        }
    })
}

#[no_mangle]
pub extern "C" fn emu_manifest_image_y(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if let Some(manifest) = &c.cartridge_manifest {
            manifest.images[index as usize].y as GmlNumber
        } else {
            0.0
        }
    })
}

#[no_mangle]
pub extern "C" fn emu_manifest_image_w(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if let Some(manifest) = &c.cartridge_manifest {
            manifest.images[index as usize].w as GmlNumber
        } else {
            0.0
        }
    })
}

#[no_mangle]
pub extern "C" fn emu_manifest_image_h(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if let Some(manifest) = &c.cartridge_manifest {
            manifest.images[index as usize].h as GmlNumber
        } else {
            0.0
        }
    })
}

#[no_mangle]
pub extern "C" fn emu_manifest_image_ox(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if let Some(manifest) = &c.cartridge_manifest {
            manifest.images[index as usize].ox as GmlNumber
        } else {
            0.0
        }
    })
}

#[no_mangle]
pub extern "C" fn emu_manifest_image_oy(index: GmlNumber) -> GmlNumber {
    State::with(|c| {
        if let Some(manifest) = &c.cartridge_manifest {
            manifest.images[index as usize].oy as GmlNumber
        } else {
            0.0
        }
    })
}

pub fn string_from_raw_unsized(mut source: *const libc::c_uchar) -> String {
    if source.is_null() {
        return "".to_owned();
    }
    let mut bytes = vec![];
    unsafe {
        while *source != 0 {
            bytes.push(*source);
            source = source.add(1);
        }
        let cstring = CString::from_vec_unchecked(bytes);
        cstring.into_string().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general() {
        emu_cpu_start(CString::new("resources").unwrap().as_ptr() as GmlString);
        while emu_cpu_resume() > GML_FALSE {}

        let vx = emu_viewport_x();
        let vy = emu_viewport_y();
        let spr = 42 as GmlNumber;
        let img = emu_sprite_image(spr);
        let sx = emu_sprite_x(spr);
        let sy = emu_sprite_y(spr);
        assert_eq!(vx as i16, 1);
        assert_eq!(vy as i16, 2);
        assert_eq!(img as i16, 3);
        assert_eq!(sx as i16, 4);
        assert_eq!(sy as i16, 5);

        let cep = string_from_raw_unsized(emu_manifest_images_atlas_path() as *const libc::c_uchar);
        let c = emu_manifest_images_count() as usize;
        let x = emu_manifest_image_x(0 as GmlNumber) as i16;
        let y = emu_manifest_image_y(0 as GmlNumber) as i16;
        let w = emu_manifest_image_w(0 as GmlNumber) as i16;
        let h = emu_manifest_image_h(0 as GmlNumber) as i16;
        let ox = emu_manifest_image_ox(0 as GmlNumber) as i16;
        let oy = emu_manifest_image_oy(0 as GmlNumber) as i16;
        assert_eq!(cep, "atlas.png");
        assert_eq!(c, 1);
        assert_eq!(x, 0);
        assert_eq!(y, 1);
        assert_eq!(w, 2);
        assert_eq!(h, 3);
        assert_eq!(ox, -1);
        assert_eq!(oy, -2);
    }
}
