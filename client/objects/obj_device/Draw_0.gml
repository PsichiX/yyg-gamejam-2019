if emu_cpu_can_resume() {
	if self.surface_dirty {
		self.surface_dirty = false;
		if surface_exists(self.surface) {
			surface_set_target(self.surface);
			var col = emu_bgcolor();
			draw_clear(make_color_rgb((col >> 16) & 0xff, (col >> 8) & 0xff, col & 0xff));
			var count = emu_sprites_count();
			for (var i = 0; i < count; ++i) {
				var img = emu_sprite_image(i);
				if img >= 0 && img < ds_list_size(self.sprites) {
					var tx = round(emu_sprite_x(i));
					var ty = round(emu_sprite_y(i));
					var data = ds_list_find_value(self.sprites, img);
					var sx = ds_list_find_value(data, 0);
					var sy = ds_list_find_value(data, 1);
					var sw = ds_list_find_value(data, 2);
					var sh = ds_list_find_value(data, 3);
					var sox = ds_list_find_value(data, 4);
					var soy = ds_list_find_value(data, 5);
					draw_sprite_general(
						self.atlas_sprite, 0,
						sx, sy, sw, sh, tx - sox, ty - soy,
						1, 1, 0,
						c_white, c_white, c_white, c_white, 1
					);
				}
			}
			surface_reset_target();
		}
	}
	if self.freezing {
		draw_rectangle_color(0, 0, 20, 20, self.bg_color, self.bg_color, self.bg_color, self.bg_color, false);
		draw_font_text(2, 6, "...", 0, floor(self.current_rom_anim_phase) % 4);
		self.current_rom_anim_phase += 0.075;
	} else if surface_exists(self.surface) {
		draw_surface(self.surface, 0, 0);
		self.current_rom_anim_phase = 0;
	}
} else {
	var count = ds_list_size(self.rom_list);
	draw_rectangle_color(0, 0, 20, 20, self.bg_color, self.bg_color, self.bg_color, self.bg_color, false);
	if count > 0 {
		draw_font_text(0, 0, "rom:", 0, 4);
		draw_sprite(spr_curr, 0, 0, 7);
		var name = ds_list_find_value(self.rom_list, self.current_rom_index);
		draw_font_text(3, 6, name, floor(self.current_rom_anim_phase), 3);
		if self.current_rom_index < count - 1 {
			name = ds_list_find_value(self.rom_list, self.current_rom_index + 1);
			draw_font_text(3, 12, name, 0, 3);
		}
		if self.current_rom_index > 0 {
			draw_sprite(spr_prev, 0, 2, 18);
		}
		if self.current_rom_index < count - 1 {
			draw_sprite(spr_next, 0, 16, 18);
		}
	} else {
		draw_font_text(1, 4, "no\nrom", 0, 6);
	}
	
	self.current_rom_anim_phase += 0.01;
}
