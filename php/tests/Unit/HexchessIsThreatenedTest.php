<?php

use Bedard\Hexchess\Hexchess;

testJson('hexchess-is-threatened', function ($spec) {
    $result = Hexchess::parse($spec['from'])->isThreatened($spec['position']);

    expect($result)->toEqual($spec['expect']);
});
