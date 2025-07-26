<?php

use Bedard\Hexchess\Constants;
use Bedard\Hexchess\San;

testJson('san-parse', function ($t) {
    if ($t['error']) {
        expect(fn () => San::from($t['san']))->toThrow(\InvalidArgumentException::class);
    } else {
        $san = San::from($t['san']);
        expect($san->from)->toBe(Constants::index($t['expect']['from']));
        expect($san->to)->toBe(Constants::index($t['expect']['to']));
        expect($san->promotion)->toBe($t['expect']['promotion']);
    }
});
