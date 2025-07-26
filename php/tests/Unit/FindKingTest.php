<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testJson('find-king', function ($t) {
    $hexchess = Hexchess::parse($t['from']);
    $king = $hexchess->findKing($t['color']);

    if (is_string($t['result'])) {
        expect($king)->toBe(Board::index($t['result']));
    } else {
        expect($king)->toBeNull();
    }
});
