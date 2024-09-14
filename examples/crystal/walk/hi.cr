puts "Hello World!"

@[Link("env")]
lib LibEnv
  fun blah(x : Int32) : Int32
end

fun add(a : Int32, b : Int32) : Int32
  a + LibEnv.blah(b)
end
