<?php

use Bedard\Hexchess\Hexchess;

testJson('get', function ($spec) {
    $hexchess = Hexchess::parse($spec['from']);
    $result = $hexchess->get($spec['position']);

    expect($result)->toBe($spec['result']);
});
