<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testYaml('board-traversal', function ($spec) {
    $hexchess = new Hexchess();
    $position = Board::index($spec['position']);

    for ($direction = 0; $direction < 12; $direction++) {
        $positions = array_map(fn ($n) => Board::position($n), Board::walk($hexchess, $position, $direction, 'w'));

        expect($positions)->toEqual($spec['expected'][$direction]);
    }
}, 'position');
