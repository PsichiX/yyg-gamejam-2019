if keyboard_check(ord("A")) || keyboard_check(vk_left) || self.hold {
	self.image_blend = c_dkgray;
} else {
	self.image_blend = c_white;
}
