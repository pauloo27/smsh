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
				actions = {
					{
						type = "print_value_to_stdout",
						value = "",
					},
					{
						type = "exit_with_code",
						value = "0",
					},
				},
			},
		},
	},
})
