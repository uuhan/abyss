local M = {}

local tools = require("tools")
local list = require("list")

function M.hello(obj)
	tools.hello(obj)
end

function M.empty() end

return M
