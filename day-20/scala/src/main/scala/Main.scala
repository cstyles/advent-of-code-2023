enum Pulse:
  case High
  case Low

enum NodeKind:
  case FlipFlop(on: Boolean)
  case Conjunction(inputs: collection.mutable.Map[String, Pulse])
  case Broadcaster

class Node(
    val name: String,
    val destinations: Array[String],
    val kind: NodeKind
):
  def with_kind(kind: NodeKind): Node = {
    Node(this.name, this.destinations, kind)
  }

  def process(source: String, pulse: Pulse): (Node, Array[Signal]) = {
    this.kind match
      case NodeKind.FlipFlop(on) => {
        pulse match
          case Pulse.Low => {
            val new_node = this.with_kind(NodeKind.FlipFlop(!on))
            val new_pulse =
              if !on
              then Pulse.High
              else Pulse.Low

            (
              new_node,
              this.destinations
                .map((dest) => Signal(this.name, dest, new_pulse))
                .toArray
            )
          }
          case Pulse.High => (this, Array())
      }
      case NodeKind.Conjunction(inputs) => {
        inputs.put(source, pulse)
        val new_pulse: Pulse =
          if inputs.values.forall(_ == Pulse.High)
          then Pulse.Low
          else Pulse.High

        (
          this,
          this.destinations
            .map((dest) => Signal(this.name, dest, new_pulse))
            .toArray
        )
      }
      case NodeKind.Broadcaster => throw new RuntimeException("unreachable?")
  }

def parse_node(line: String): Node = {
  val splitter = line.split(" -> ")
  val node = splitter(0)
  val destinations = splitter(1).split(", ")

  val (name, kind) = node(0) match
    case 'b' => ("broadcaster", NodeKind.Broadcaster)
    case '%' => (node.substring(1), NodeKind.FlipFlop(false))
    case '&' =>
      (node.substring(1), NodeKind.Conjunction(collection.mutable.Map()))

  Node(name, destinations, kind)
}

case class Signal(
    source: String,
    dest: String,
    pulse: Pulse
)

@main def main: Unit =
  // val input = io.Source.fromFile("../test_input.txt")
  // val input = io.Source.fromFile("../test_input2.txt")
  val input = io.Source.fromFile("../input.txt")

  val nodes = collection.mutable.Map[String, Node]()
  for (node <- input.getLines().map(parse_node)) {
    nodes.put(node.name, node)
  }

  val input_map = scala.collection.mutable.Map[String, List[String]]()
  for (node <- nodes.values) {
    for (destination <- node.destinations) {
      val inputs = input_map.getOrElse(destination, List[String]())
      input_map(destination) = node.name :: inputs
    }
  }

  // println(input_map)

  for (node <- nodes.values) {
    node.kind match
      case NodeKind.Conjunction(inputs) => {
        inputs ++= input_map.get(node.name).get.map((_, Pulse.Low))
      }
      case _ => ()
  }

  val queue = collection.mutable.Queue[Signal]()
  var low_sent = 0
  var high_sent = 0
  var button_press = 0

  val precursors = collection.mutable.Map[String, Option[Long]](
    ("tf", None),
    ("db", None),
    ("vq", None),
    ("ln", None)
  )

  while (true) {
    button_press += 1
    low_sent += 1

    val broadcaster = nodes.get("broadcaster").get
    queue.addAll(
      broadcaster.destinations.map((dest) =>
        Signal("broadcaster", dest, Pulse.Low)
      )
    )

    while (queue.nonEmpty) {
      val signal = queue.dequeue

      signal.pulse match
        case Pulse.High => {
          high_sent += 1
          precursors.get(signal.source) match
            case Some(Some(_existing)) => ()
            case Some(None) => precursors.put(signal.source, Some(button_press))
            case _          => ()
        }
        case Pulse.Low => low_sent += 1

      if precursors.values.forall(_.nonEmpty)
      then {
        val part2 = precursors.values.map(_.get).product
        println(s"part2 = $part2")
        System.exit(0)
      } else ()

      if signal.dest == "output"
      then ()
      else {
        nodes.get(signal.dest) match
          case Some(node) => {
            val (processed, result) = node.process(signal.source, signal.pulse)
            nodes.put(signal.dest, processed)
            queue.addAll(result)
          }
          case _ => () // TODO: part 2?
      }
    }

    if button_press == 1000
    then {
      val part1 = high_sent * low_sent
      println(s"part1 = $part1")
    }
  }
