<?php

use Bedard\Hexchess\Hexchess;

testYaml('current-moves', function ($spec) {
    $hexchess = Hexchess::parse($spec['hexchess']);
    $moves = array_map(fn ($san) => (string) $san, $hexchess->currentMoves());

    sort($moves);
    sort($spec['expected']);

    expect($moves)->toEqual($spec['expected']);
});
