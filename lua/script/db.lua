-- print(PG:query([[select tick from ticks limit 1]])[1])
-- print(PG:query([[select tick::text from ticks limit 1]])[1])

local config = require("database.postgres.config")
local db = postgres.new(config)

local count, err

count, err = db:execute([[ select new() ]])
print(count, err)

count, err = db:execute([[ select now() ]])
print(count, err)
