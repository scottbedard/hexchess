<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testYaml('moves-from', function ($spec) {
    $hexchess = Hexchess::parse($spec['hexchess']);
    $position = Board::index($spec['position']);
    $moves = array_map(fn ($san) => Board::position($san->to), $hexchess->movesFrom($position));

    sort($moves);
    sort($spec['expected']);

    expect($moves)->toEqual($spec['expected']);
});
