<?php

use Bedard\Hexchess\Hexchess;

testYaml('is-threatened', function ($spec) {
    $result = Hexchess::parse($spec['hexchess'])->isThreatened($spec['position']);

    expect($result)->toEqual($spec['expected']);
});
