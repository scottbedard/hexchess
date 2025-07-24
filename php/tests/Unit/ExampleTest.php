<?php

use Bedard\Hexchess\Hexchess;

test('example', function () {
    $hexchess = new Hexchess();

    expect($hexchess->add(1, 2))->toBe(3);
});
