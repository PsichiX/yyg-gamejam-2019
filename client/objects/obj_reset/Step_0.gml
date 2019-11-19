if keyboard_check(ord("R")) || self.hold {
	self.image_blend = c_dkgray;
} else {
	self.image_blend = c_white;
}
