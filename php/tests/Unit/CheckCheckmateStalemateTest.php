<?php

use Bedard\Hexchess\Hexchess;

testJson('check-checkmate-stalemate', function ($spec) {
    $hexchess = Hexchess::parse($spec['from']);

    expect($spec['check'])->toBe($hexchess->isCheck());
    expect($spec['checkmate'])->toBe($hexchess->isCheckmate());
    expect($spec['stalemate'])->toBe($hexchess->isStalemate());
});
