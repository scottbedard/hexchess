<?php

use Bedard\Hexchess\Constants;
use Bedard\Hexchess\Hexchess;

test('init', function () {
    $hexchess = Hexchess::init();

    expect($hexchess->__toString())->toBe(Constants::INITIAL_POSITION);
});
