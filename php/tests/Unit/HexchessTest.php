<?php

use Bedard\Hexchess\Constants;
use Bedard\Hexchess\Exceptions\IllegalMoveException;
use Bedard\Hexchess\Hexchess;

test('init', function () {
    $hexchess = Hexchess::init();

    expect($hexchess->__toString())->toBe(Constants::INITIAL_POSITION);
});

test('IllegalMoveException', function () {
    $hexchess = Hexchess::init();

    expect(fn () => $hexchess->applyMove('a1a6'))->toThrow(IllegalMoveException::class);
});
