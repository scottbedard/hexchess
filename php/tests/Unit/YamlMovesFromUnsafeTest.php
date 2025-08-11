<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testYaml('moves-from-unsafe', function ($spec) {
    $hexchess = Hexchess::parse($spec['hexchess']);
    $results = $hexchess->movesFromUnsafe(Board::index($spec['position']));

    $arr = array_map(fn ($san) => (string) $san, $results);
    $expected = array_map(fn ($san) => (string) $san, $spec['expected']);
    sort($expected);
    sort($arr);

    expect($arr)->toBe($expected);
});
