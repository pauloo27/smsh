window({
	title = "SMSH: Prompt",
	enable_esc_as_exit = true,
	present = true,
	root = {
		type = "container",
		orientation = "horizontal",
		children = {
			{
				type = "entry",
				text = "",
				tooltip = "Type here",
				action = function(value)
					print(value)
					os.exit(0)
				end,
			},
		},
	},
})
