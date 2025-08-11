<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

function testMoves($spec)
{
    $hexchess = Hexchess::parse($spec['hexchess']);
    $position = Board::index($spec['position']);
    $moves = array_map(fn ($san) => (string) $san, $hexchess->movesFrom($position));

    sort($moves);
    sort($spec['expected']);

    expect($moves)->toEqual($spec['expected']);
}

testYaml('moves-from', fn ($spec) => testMoves($spec));

testYaml('moves-king', fn ($spec) => testMoves($spec));

testYaml('moves-knight', fn ($spec) => testMoves($spec));

testYaml('moves-pawn', fn ($spec) => testMoves($spec));

testYaml('moves-straight-line', fn ($spec) => testMoves($spec));
