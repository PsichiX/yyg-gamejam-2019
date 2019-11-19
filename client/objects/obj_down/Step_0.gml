if keyboard_check(ord("S")) || keyboard_check(vk_down) || self.hold {
	self.image_blend = c_dkgray;
} else {
	self.image_blend = c_white;
}
