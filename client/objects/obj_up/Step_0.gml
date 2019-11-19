if keyboard_check(ord("W")) || keyboard_check(vk_up) || self.hold {
	self.image_blend = c_dkgray;
} else {
	self.image_blend = c_white;
}
