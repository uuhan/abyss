local M = {}

local config = {}
local ticks = {}

-- 品种
M.products = {}
-- 合约
M.instruments = {
  "fu2405",
}

-- 行情接收
function M.on_continous(ticks)
  local tick = ticks.fu2405
  local price = tick.price

  for id, tick in ipairs(ticks) do
    ticks[id] = tick
  end
end

function M.on_no_trading() end

function M.on_before_trading() end

function M.on_auction_ordering() end

function M.on_auction_balance() end

function M.on_auction_match() end

function M.on_closed() end

return M
