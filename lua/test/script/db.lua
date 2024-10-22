local config = require("database.postgres.config")
local db = postgres.new(config)

local count, err

count, err = db:execute([[ select now() ]])
if err then
  print(err)
end

local begin = os.time()

local ticks = db:load(
  "fu2409",
  os.time({ year = 2024, month = 1, day = 5, hour = 8, min = 59 }),
  os.time({ year = 2024, month = 1, day = 5, hour = 9, min = 10 })
)

if ticks then
  print("Get Ticks: " .. #ticks)
  print("Time: " .. os.time() - begin)
end
