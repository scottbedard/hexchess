<?php

use Bedard\Hexchess\Constants;

describe('constants', function () {
    test('index', function () {
        expect(Constants::index('f11'))->toBe(0);
        expect(Constants::index(0))->toBe(0);
    });

    test('index throws error', function () {
        expect(fn () => Constants::index('f12'))->toThrow(\InvalidArgumentException::class);
    });
});
