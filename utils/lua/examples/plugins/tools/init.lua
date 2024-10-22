local M = {}

function M.hello(obj)
  print("hello: ", obj.hello)
end

function M:say()
  print(self.config)
end

M.config = { "a", "b", "c", "d", "e", "f" }

return M
