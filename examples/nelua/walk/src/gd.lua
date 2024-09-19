local gd = {}

local parse_value
local section_pattern = "^%[(.+)%]$"

local function trim(s)
  return s:match("^[,%s]*(.-)[,%s]*$")
end

local function parse_array(file, text)
  text = trim(text)
  -- print("array", text)
  local rest, value
  local values = {}
  while true do
    -- print(text)
    rest = text:match("](.*)")
    if rest then
      goto done
    end
    value, text = parse_value(file, text)
    if value then
      table.insert(values, value)
    else
      rest = ""
      goto done
    end
    if text == "" then
      text = file:read("*line")
      if not text then
        rest = ""
        goto done
      end
    end
  end
  ::done::
  return values, rest
end

local function parse_call(file, type, args)
  local values = {}
  for match in args:gmatch("[^,%s]+") do
    -- print(file, match, parse_value)
    local value = parse_value(file, match)
    table.insert(values, value)
  end
  return { type = type, args = values }
end

parse_value = function(file, text)
  text = trim(text)
  -- Number.
  local num, rest = text:match("^([%d.]+)(.*)")
  if num then
    return tonumber(num), rest
  end
  -- String. Presume no quotes inside strings for now.
  local str, rest = text:match("^&?\"(.-)\"(.*)")
  if str then
    return str, rest
  end
  -- Call.
  local type, args, rest = text:match("^(%w+)%((.-)%)(.*)")
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
  -- TODO
  return text, ""
end

local function parse_key_value(file, line)
  -- print(line)
  local key, value = line:match("^(%S+)%s*=%s*(.+)$")
  if not key or not value then
    return nil, nil
  end
  value = parse_value(file, value)
  return key, value
end

local function parse_section(file)
  local section_data = {}
  for line in file:lines() do
    if line:match("^%s*$") or line:match("^%s*;") then
      goto continue
    end
    if line:match(section_pattern) then
      file:seek("cur", - #line - 1)
      goto done
    end
    -- Parse key value pairs.
    local key, value = parse_key_value(file, line)
    if key and value then
      section_data[key] = value
    end
    ::continue::
  end
  ::done::
  return section_data
end

function gd.resource_load(file_path)
  local file <close> = io.open(file_path, "r")
  if not file then
    error("Can't open: " .. file_path)
  end
  local resource = {}
  for line in file:lines() do
    local section = line:match(section_pattern)
    if section then
      resource[section] = parse_section(file)
    end
  end
  return resource
end

return gd
