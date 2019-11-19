/// @func draw_font_text(x, y, text, start, length)

var tx = argument0;
var ty = argument1;
for (var i = 0; i < argument4; ++i) {
	var pos = (argument3 + i) % (string_length(argument2) + 2) + 1;
	var char = string_char_at(argument2, pos);
	if char == "\n" {
		tx = argument0;
		ty += 6;
	} else {
		var idx = font_get_tile_index(char);
		if idx >= 0 {
			var c = idx % 6;
			var r = floor(idx / 6);
			draw_sprite_part(spr_font, 0, c * 5, r * 5, 5, 5, tx, ty);
		}
		tx += 6;
	}
}