if keyboard_check(ord("D")) || keyboard_check(vk_right) || self.hold {
	self.image_blend = c_dkgray;
} else {
	self.image_blend = c_white;
}
