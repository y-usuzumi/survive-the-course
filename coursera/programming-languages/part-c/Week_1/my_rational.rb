class MyRational
  attr_reader :num, :den
  protected :num, :den

  def initialize(num, den=1)
    if den == 0
      raise "Denominator cannot be zero"
    elsif den < 0
      @num = -num
      @den = -den
    else
      @num = num
      @den = den
    end
    reduce
  end

  def reduce
    gcd = gcd(@num, @den)
    @num /= gcd
    @den /= gcd
  end

  def add! other
    @num = @num * other.den + other.num * @den
    @den *= other.den
    reduce
    self
  end

  def + other
    new_rational = self.class.new(@num, @den)
    new_rational.add! other
  end

  def to_s
    s = @num.to_s
    s += ("/" + @den.to_s) if @den != 1
    s
  end

  def inspect
    to_s
  end

  private
  def gcd(a, b)
    if a < b
      gcd(b, a)
    elsif b != 0
      gcd(b, a % b)
    else
      a
    end
  end
end
