---@alias ContainerOrientation "vertical"|"horizontal"

---@class Action : fun(value: any)

---@class Component
---@field type "label"|"button"|"entry"|"container"

---@class LabelComponent : Component
---@field type "label"
---@field text string
---@field tooltip? string

---@class ButtonComponent : Component
---@field type "button"
---@field text string
---@field tooltip? string
---@field action? Action

---@class EntryComponent : Component
---@field type "entry"
---@field text string
---@field tooltip? string
---@field action? Action

---@class ContainerComponent : Component
---@field type "container"
---@field orientation ContainerOrientation
---@field children Component[]

---@class Window
---@field title string Window title
---@field width? integer Window width in pixels
---@field height? integer Window height in pixels
---@field root Component Root component of the window
---@field enable_vim_keys? boolean Enable vim keybindings
---@field enable_esc_as_exit? boolean Allow ESC to exit
---@field present? boolean Present the window
---@field exit_on_close? boolean Exit on window close
---@field exit_code? integer Exit code to use

---Create a new window
---@param win_def Window The window definition
---@return nil
local function new_window(win_def)
	window(win_def)
end

-- Example usage with type checking:
new_window({
	title = "SMSH: Typed Wrapper Example",
	enable_vim_keys = true,
	enable_esc_as_exit = true,
	present = true,
	root = {
		type = "container",
		orientation = "vertical",
		children = {
			{
				type = "label",
				text = "Hello from typed wrapper!",
				tooltip = "This is a label",
			},
			{
				type = "button",
				text = "Click me",
				tooltip = "Press this button",
				action = function()
					print("Button clicked!")
					os.exit(0)
				end,
			},
		},
	},
})
