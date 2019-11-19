/// @func font_get_tile_index(character)

var idx = ord(argument0);
if idx >= 65 && idx <= 90 {
	return idx - 65;
} else if idx >= 97 && idx <= 122 {
	return idx - 97;
} else if idx >= 48 && idx <= 57 {
	return idx - 48 + 26;
} else if idx == 58 {
	return 36;
} else if idx == 95 {
	return 37;
} else if idx == 45 {
	return 38;
} else if idx == 46 {
	return 39;
} else if idx == 44 {
	return 40;
} else if idx == 34 {
	return 41;
} else {
	return -1;
}