<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testJson('moves-straight-line', function ($spec) {
    $hexchess = Hexchess::parse($spec['from']);
    $results = $hexchess->movesFromUnsafe(Board::index($spec['position']));

    expect(array_map(fn ($san) => (string) $san, $results))->toBe($spec['expect']);
});
