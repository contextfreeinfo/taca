-- This script should work in ordinary Lua.

local pub = {}

local parse_value
local header_pattern = "^%[(.+)%]$"

local function trim(s)
  return s:match("^[,%s]*(.-)[,%s]*$")
end

local function read_line(file, text)
  text = trim(text)
  if text ~= "" then
    return text
  end
  while true do
    local line = file:read("*line")
    if not line then
      return nil
    end
    line = trim(line)
    if not (line:match("^%s*$") or line:match("^%s*;")) then
      return line
    end
  end
end

local function parse_array(file, text)
  local rest, value
  local values = {}
  while true do
    text = read_line(file, text)
    if not text then
      break
    end
    rest = text:match("](.*)")
    if rest then
      break
    end
    value, text = parse_value(file, text)
    if value then
      table.insert(values, value)
    else
      rest = ""
      break
    end
  end
  return values, rest
end

local function parse_call(file, type, args)
  local values = {}
  -- This splits on spaces in strings. TODO Parse like arrays.
  for match in args:gmatch("[^,%s]+") do
    local value = parse_value(file, match)
    table.insert(values, value)
  end
  return { type = type, args = values }
end

local function parse_table(file, text)
  local key, rest, value
  local values = {}
  while true do
    text = read_line(file, text)
    if not text then
      break
    end
    rest = text:match("}(.*)")
    if rest then
      break
    end
    key, rest = text:match("^\"([^\"]*)\"%s*:%s*(.*)")
    if key then
      value, text = parse_value(file, rest)
      values[key] = value
    else
      rest = ""
      break
    end
  end
  return values, rest
end

parse_value = function(file, text)
  text = trim(text)
  -- Number.
  local num, rest = text:match("^([%d.]+)(.*)")
  if num then
    return tonumber(num), rest
  end
  -- String. Presume no quotes nor escapes inside strings for now.
  local str, rest = text:match("^&?\"(.-)\"(.*)")
  if str then
    return str, rest
  end
  -- Call. Presume only numbers & paren-less strings on single line for now.
  local type, args, rest = text:match("^([%w_]+)%((.-)%)(.*)")
  if type then
    return parse_call(file, type, args), rest
  end
  -- Array.
  local rest = text:match("^%[(.*)")
  if rest then
    local value, rest = parse_array(file, rest)
    return value, rest
  end
  -- Table.
  local rest = text:match("^{(.*)")
  if rest then
    local value, rest = parse_table(file, rest)
    return value, rest
  end
  return text, ""
end

local function parse_key_value(file, line)
  local key, value = line:match("^(%S+)%s*=%s*(.+)$")
  if not key or not value then
    return nil, nil
  end
  value = parse_value(file, value)
  return key, value
end

local function parse_section(file)
  local section_data = {}
  while true do
    local line = read_line(file, "")
    if not line then
      break
    end
    if line:match(header_pattern) then
      file:seek("cur", - #line - 1)
      break
    end
    -- Parse key value pairs.
    local key, value = parse_key_value(file, line)
    if key and value then
      section_data[key] = value
    end
  end
  return section_data
end

local function parse_header(header)
  local attrs = {}
  local tag, rest = header:match("^([%w_]+)%s*(.*)")
  -- Presume no nested quotes for now.
  for key, value in rest:gmatch("([%w_]+)%s*=%s*\"(.-)\"") do
    attrs[key] = value
  end
  return tag, attrs
end

local function print_table(t, indent)
  indent = indent or ""
  for k, v in pairs(t) do
    if type(v) == "table" then
      print(indent .. k .. " = {")
      print_table(v, indent .. "  ")
      print(indent .. "}")
    else
      print(indent .. k .. " = " .. tostring(v))
    end
  end
end
pub.print_table = print_table

-- Variation on http://lua-users.org/wiki/BaseSixtyFour
function pub.base64_decode(data)
  local b = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'
  return (data:gsub('.', function(x)
    if (x == '=') then return '' end
    local r, f = '', (b:find(x) - 1)
    for i = 6, 1, -1 do
      r = r .. (f % 2 ^ i - f % 2 ^ (i - 1) > 0 and '1' or '0')
    end
    return r
  end):gsub('%d%d%d?%d?%d?%d?%d?%d?', function(x)
    if (#x ~= 8) then return '' end
    return string.char(tonumber(x, 2))
  end))
end

function pub.resource_load(file_path)
  local file <close> = io.open(file_path, "r")
  if not file then
    error("Can't open: " .. file_path)
  end
  local resource = {}
  for line in file:lines() do
    local header = line:match(header_pattern)
    if header then
      local tag, attrs = parse_header(trim(header))
      local key
      if tag == "gd_resource" or tag == "gd_scene" then
        resource.uid = attrs.uid
        key = tag
      else
        key = attrs.id or attrs.name or tag
      end
      resource[key] = {
        tag = tag,
        attrs = attrs,
        body = parse_section(file),
      }
    end
  end
  -- print("--------------------", file_path)
  -- print_table(resource)
  return resource
end

return pub
