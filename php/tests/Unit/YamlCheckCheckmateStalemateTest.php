<?php

use Bedard\Hexchess\Hexchess;

testYaml('check-checkmate-stalemate', function ($spec) {
    $hexchess = Hexchess::parse($spec['hexchess']);

    expect($spec['check'])->toBe($hexchess->isCheck());
    expect($spec['checkmate'])->toBe($hexchess->isCheckmate());
    expect($spec['stalemate'])->toBe($hexchess->isStalemate());
});
