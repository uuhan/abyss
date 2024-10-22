local M = {}

M.instruments = {
  "fu2409",
}

function M.on_tick(tick)
  print("On Tick: ", tick.fu2409)
end

return M
