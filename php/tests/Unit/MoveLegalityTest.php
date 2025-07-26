<?php

use Bedard\Hexchess\Hexchess;

testJson('move-legality', function ($spec) {
    $hexchess = Hexchess::parse($spec['from']);

    $result = null;

    try {
        $result = $hexchess->isLegal($spec['san']);
    } catch (\Exception $e) {
        $result = false;
    }

    expect($result)->toBe($spec['result']);
});
