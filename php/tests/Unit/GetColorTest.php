<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

// ...
testJson('get-color', function ($spec) {
    $hexchess = Hexchess::parse($spec['from']);
    $moves = array_map(fn ($n) => Board::position($n), $hexchess->getColor($spec['color']));

    expect($moves)->toEqual($spec['result']);
});
