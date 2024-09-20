-- This script is specifically for Nelua macro usage.

local macros = {}

local n = require "nelua.aster"

local function find(array, fun)
  for index, value in ipairs(array) do
    if fun(value) then
      return value, index
    end
  end
end

local function map(array, fun)
  local new_array = {}
  for i, v in ipairs(array) do
    new_array[i] = fun(v)
  end
  return new_array
end

function macros.animation_build(resource, name)
  local animations = resource.resource.body.animations
  local animation = find(animations, function(anim)
    return anim.name == name
  end)
  return n.InitList(map(animation.frames, function(frame)
    local region = resource[frame.texture.args[1]].body.region.args
    return n.InitList{
      n.Pair{"pos", n.InitList{n.Number{region[1]}, n.Number{region[2]}}},
      n.Pair{"size", n.InitList{n.Number{region[3]}, n.Number{region[4]}}},
    }
  end))
end

return macros
