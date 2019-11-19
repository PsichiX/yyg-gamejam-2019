/// @description load rom listing

ds_list_clear(self.rom_list);
if directory_exists(self.roms_directory) {
	var f = file_find_first(self.roms_directory + "/*", fa_directory);
	while string_length(f) > 0 {
		ds_list_add(self.rom_list, filename_name(f));
		f = file_find_next();
	}
	file_find_close();
}
self.current_rom_index = 0;
self.current_rom_anim_phase = 0;
