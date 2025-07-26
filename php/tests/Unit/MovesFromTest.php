<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testJson('moves-from', function ($spec) {
    $hexchess = Hexchess::parse($spec['from']);
    $results = $hexchess->movesFrom(Board::index($spec['position']));

    $arr = array_map(fn ($san) => (string) $san, $results);
    $expected = array_map(fn ($san) => (string) $san, $spec['expect']);
    sort($expected);
    sort($arr);

    expect($arr)->toBe($expected);
});
