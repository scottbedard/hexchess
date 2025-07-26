<?php

use Bedard\Hexchess\Hexchess;

testJson('hexchess-to-string', function ($spec) {
    $hexchess = new Hexchess();
    $hexchess->board = $spec['from']['board'];
    $hexchess->ep = $spec['from']['ep'];
    $hexchess->fullmove = $spec['from']['fullmove'];
    $hexchess->halfmove = $spec['from']['halfmove'];
    $hexchess->turn = $spec['from']['turn'];

    expect((string) $hexchess)->toEqual($spec['result']);
});
