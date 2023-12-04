SIZE = 140

struct Point
  property y, x

  def initialize(@y : Int32, @x : Int32); end

  def neighbors
    [
      up_left, up, up_right, left, right, down_left, down, down_right,
    ].compact
  end

  def up_left
    @y.checked_pred.try do |y|
      @x.checked_pred.try do |x|
        self.class.new y, x
      end
    end
  end

  def up
    @y.checked_pred.try { |y| self.class.new y, @x }
  end

  def up_right
    @y.checked_pred.try do |y|
      @x.checked_succ.try do |x|
        self.class.new y, x
      end
    end
  end

  def left
    @x.checked_pred.try { |x| self.class.new @y, x }
  end

  def right
    @x.checked_succ.try { |x| self.class.new @y, x }
  end

  def down_left
    @y.checked_succ.try do |y|
      @x.checked_pred.try do |x|
        self.class.new y, x
      end
    end
  end

  def down
    @y.checked_succ.try { |y| self.class.new y, @x }
  end

  def down_right
    @y.checked_succ.try do |y|
      @x.checked_succ.try do |x|
        self.class.new y, x
      end
    end
  end
end

struct Int32
  def checked_pred
    if zero?
      nil
    else
      pred
    end
  end

  def checked_succ
    if self >= SIZE - 1
      nil
    else
      succ
    end
  end
end
