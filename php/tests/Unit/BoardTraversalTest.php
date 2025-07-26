<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testJson('board-traversal', function ($t) {
    $hexchess = Hexchess::parse($t['hexchess']);

    $path = Board::walk($hexchess, Board::index($t['from']), $t['direction'], $t['color']);

    expect(array_map(fn ($index) => Board::position($index), $path))->toEqual($t['result']);
});
