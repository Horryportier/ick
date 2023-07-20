#! /usr/bin/lua

local json = require 'dkjson'
DefWindow = {

    id = "aoetunhtnehou",
    name =  "Test",

    width = 200,
    height = 400,

    focusable = true,
    resizable = false,

    opacity = 0.9,
    css_name = "test",
    css_classes = "",
}
DefWindow['width']= math.random(10, 200)


local function save_file(content)
  local file,err = io.open("DefWindow.json", 'w')
  if file then
    file:write(content)
    file:close()
  else
    print("error:", err)
  end
end


local socket = require("posix.sys.socket")
local unistd = require("posix.unistd")
local fd = assert(socket.socket(socket.AF_UNIX, socket.SOCK_STREAM, 0))
assert(socket.connect(fd, {family = socket.AF_UNIX, path = "/tmp/test_socket"}))

-- local w, h = io.read("*n", "*n")
--
-- DefWindow.width = tonumber(w)
-- DefWindow.height = tonumber(h)
J = json.encode(DefWindow)
save_file(J)

assert(socket.send(fd, J))

unistd.close(fd)
