if emu_cpu_can_resume() {
	if keyboard_check_pressed(ord("R")) || obj_reset.hold {
		emu_cpu_stop();
	} else {
		var input_up = (keyboard_check(ord("W")) || keyboard_check(vk_up) || obj_up.hold) << 0;
		var input_down = (keyboard_check(ord("S")) || keyboard_check(vk_down) || obj_down.hold) << 1;
		var input_left = (keyboard_check(ord("A")) || keyboard_check(vk_left) || obj_left.hold) << 2;
		var input_right = (keyboard_check(ord("D")) || keyboard_check(vk_right) || obj_right.hold) << 3;
		var input_cross = (keyboard_check(vk_space) || obj_cross.hold) << 4;
		var input_circle = (keyboard_check(vk_enter) || obj_circle.hold) << 5;
		var input = input_up | input_down | input_left | input_right | input_cross | input_circle;
		emu_set_input(input);
		
		var st = current_time;
		var ct = st;
		do {
			emu_cpu_resume();
			var errors = emu_last_errors();
			if string_length(errors) > 0 {
				show_debug_message(errors);
			}
			ct = current_time;
		} until (!emu_cpu_can_resume() || emu_cpu_is_halt() || ct - st > 50);
		
		var vx = clamp(emu_viewport_x(), 0, 20);
		var vy = clamp(emu_viewport_y(), 0, 20);
		camera_set_view_pos(view_camera[0], vx, vy);
		self.surface_dirty = emu_cpu_is_halt();
		self.freezing = !self.surface_dirty && ct - st > 50;
		emu_cpu_clear_halt();
	}
} else {
	event_perform(ev_other, ev_user1);
	
	if keyboard_check_pressed(ord("W")) || keyboard_check_pressed(vk_up) || obj_up.hold {
		self.current_rom_index = max(0, self.current_rom_index - 1);
		self.current_rom_anim_phase = 0;
	}
	if keyboard_check_pressed(ord("S")) || keyboard_check_pressed(vk_down) || obj_down.hold {
		self.current_rom_index = min(ds_list_size(self.rom_list) - 1, self.current_rom_index + 1);
		self.current_rom_anim_phase = 0;
	}
	if keyboard_check_pressed(vk_space) || obj_cross.hold {
		self.current_rom_anim_phase = 0;
		var rom = ds_list_find_value(self.rom_list, self.current_rom_index);
		var path = self.roms_directory + "/" + rom;
		if emu_cpu_start(path) {
			var atlas_path = path + "/" + emu_manifest_images_atlas_path();
			if file_exists(atlas_path) {
				self.atlas_sprite = sprite_add(atlas_path, 1, false, false, 0, 0);
				var count = emu_manifest_images_count();
				for (var i = 0; i < count; ++i) {
					var data = ds_list_create();
					ds_list_add(data, emu_manifest_image_x(i));
					ds_list_add(data, emu_manifest_image_y(i));
					ds_list_add(data, emu_manifest_image_w(i));
					ds_list_add(data, emu_manifest_image_h(i));
					ds_list_add(data, emu_manifest_image_ox(i));
					ds_list_add(data, emu_manifest_image_oy(i));
					ds_list_add(self.sprites, data);
				}
				self.surface = surface_create(40, 40);
			} else {
				show_debug_message("Sprite atlas does not exists: " + atlas_path);
				emu_cpu_stop();
			}
		} else {
			show_debug_message("Could not run ROM: " + rom);
		}
		show_debug_message(emu_last_errors());
	}
}