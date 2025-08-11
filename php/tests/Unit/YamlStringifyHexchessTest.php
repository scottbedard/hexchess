<?php

use Bedard\Hexchess\Hexchess;

testYaml('stringify-hexchess', function ($spec) {
    $hexchess = new Hexchess();
    $hexchess->board = $spec['hexchess']['board'];
    $hexchess->ep = $spec['hexchess']['ep'];
    $hexchess->fullmove = $spec['hexchess']['fullmove'];
    $hexchess->halfmove = $spec['hexchess']['halfmove'];
    $hexchess->turn = $spec['hexchess']['turn'];

    expect((string) $hexchess)->toBe($spec['expected']);
});
