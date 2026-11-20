# SigmaOS theme template for Hyprland
# Copied to ~/.config/hypr/hyprland.conf by sigmaos-theme-system

env = XCURSOR_SIZE,24
env = TERMINAL,foot
env = BROWSER,chromium
env = EDITOR,micro
env = MOD,SUPER

general {
	gaps_in = 5
	gaps_out = 10
	border_size = 2
	layout = dwindle
}

decoration {
	rounding = 10
	blur = yes
	blur_size = 8
	blur_passes = 7
	active_opacity = 1.0
	inactive_opacity = 0.95
	drop_shadow = yes
}

input {
	follow_mouse = yes
	mouse_move = $MOD
	mouse_resize = $MOD shift
}

exec-once = waybar
exec-once = mako

bind = $MOD SHIFT, Q, exec, hyprctl kill
bind = $MOD, Return, exec, $TERMINAL
bind = $MOD, D, exec, wofi --show drun
