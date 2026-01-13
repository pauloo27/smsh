local function confirm(title, message)
	return window({
		title = title or "SMSH: Confirm",
		enable_vim_keys = true,
		present = true,
		enable_esc_as_exit = true,
		root = {
			type = "container",
			orientation = "vertical",
			children = {
				{
					type = "label",
					text = message or "Are you sure?",
					tooltip = "",
				},
				{
					type = "button",
					text = "Yes",
					tooltip = "Yes!",
					actions = {
						{
							type = "exit_with_code",
							value = "0",
						},
					},
				},
				{
					type = "button",
					text = "No",
					tooltip = "No!",
					actions = {
						{
							type = "exit_with_code",
							value = "1",
						},
					},
				},
			},
		},
	})
end

confirm("SMSH: Confirm", "Are you sure?")
