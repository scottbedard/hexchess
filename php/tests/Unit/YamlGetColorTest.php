<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testYaml('get-color', function ($spec) {
  $hexchess = Hexchess::parse($spec['hexchess']);
  $moves = array_map(fn ($n) => Board::position($n), $hexchess->getColor($spec['color']));

  expect($moves)->toEqual($spec['expected']);
});
