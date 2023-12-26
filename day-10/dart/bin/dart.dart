import 'package:dart/dart.dart' as dart;
import 'dart:io';

void main(List<String> arguments) async {
  // final input = await File('../test_input.txt').readAsString();
  final input = await File('../input.txt').readAsString();

  final grid = dart.Grid(input.trimRight());
  final start = grid.start();
  final pipe = dart.part1(grid, start);
}
