class Hello
  def my_first_method
    puts "Hello, World!"
  end
end

x = Hello.new
x.my_first_method

class Foo
  Alpha = 1

  def initialize
    @@foo = 0
  end

  def incr_foo
    @@foo += 1
  end

  def foo
    @@foo
  end

  def alpha
    @@Alpha
  end

  def beta
    @@beta
  end
end
