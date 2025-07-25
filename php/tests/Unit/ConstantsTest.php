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

    test('position', function () {
        expect(Constants::position(0))->toBe('f11');
        expect(Constants::position('f11'))->toBe('f11');
    });

    test('position throws error', function () {
        expect(fn () => Constants::position('f12'))->toThrow(\InvalidArgumentException::class);
    });
});
