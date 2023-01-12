class Base
  private
  def foo
    "Hello"
  end

  public
  def to_s
    foo
  end
end

class Foo < Base
  # NOTE: Why does it not prevent me from overriding the private base method?
  def foo
    "FOO!!!"
  end
end
