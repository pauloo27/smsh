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
function new_window(win_def)
	window(win_def)
end

---Load a CSS file for styling
---@param path string Path to the CSS file
---@return nil
function load_stylesheet(path)
	load_css(path)
end
