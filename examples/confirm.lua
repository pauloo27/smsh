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
					action = function()
						os.exit(0)
					end,
				},
				{
					type = "button",
					text = "No",
					tooltip = "No!",
					action = function(value)
						os.exit(1)
					end,
				},
			},
		},
	})
end

confirm("SMSH: Confirm", "Are you sure?")
