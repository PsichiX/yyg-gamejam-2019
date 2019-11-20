self.roms_directory = working_directory + "roms";
self.rom_list = ds_list_create();
self.current_rom_index = 0;
self.current_rom_anim_phase = 0;
self.bg_color = c_navy;
self.atlas_sprite = -1;
self.sprites = ds_list_create();
self.surface = -1;
self.surface_dirty = false;
self.freezing = false;

event_perform(ev_other, ev_user0);
