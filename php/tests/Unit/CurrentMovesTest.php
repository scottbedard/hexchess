<?php

use Bedard\Hexchess\Hexchess;

testJson('current-moves', function ($spec) {
    $hexchess = Hexchess::parse($spec['from']);
    $arr = array_map(fn ($san) => (string) $san, $hexchess->currentMoves());
    $expected = array_map(fn ($san) => (string) $san, $spec['result']);
    sort($expected);
    sort($arr);

    expect($arr)->toBe($expected);
});
