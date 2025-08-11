<?php

use Bedard\Hexchess\Hexchess;

testYaml('move-legality', function ($spec) {
    $hexchess = Hexchess::parse($spec['hexchess']);

    $actual = null;

    try {
        $actual = $hexchess->isLegal($spec['san']);
    } catch (\Exception $e) {
        $actual = false;
    }

    expect($actual)->toBe($spec['expected']);
});
