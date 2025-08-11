<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

testYaml('find-king', function ($spec) {
    $hexchess = Hexchess::parse($spec['hexchess']);

    $king = $hexchess->findKing($spec['color']);

    if (is_string($spec['expected'])) {
        expect(Board::position($king))->toBe($spec['expected']);
    } else {
        expect($king)->toBeNull();
    }
});
