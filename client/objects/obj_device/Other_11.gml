/// @description release resources

if self.atlas_sprite >= 0 {
	sprite_delete(self.atlas_sprite);
	self.atlas_sprite = -1;
}

if !ds_list_empty(self.sprites) {
	for (var i = 0; i < ds_list_size(self.sprites); ++i) {
		ds_list_destroy(ds_list_find_value(self.sprites, i));
	}
	ds_list_clear(self.sprites);
}

if surface_exists(self.surface) {
	surface_free(self.surface);
	self.surface = -1;
}
