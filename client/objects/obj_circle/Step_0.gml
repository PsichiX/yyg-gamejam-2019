if keyboard_check(vk_enter) || self.hold {
	self.image_blend = c_dkgray;
} else {
	self.image_blend = c_white;
}
