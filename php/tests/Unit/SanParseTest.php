<?php

use Bedard\Hexchess\San;
use Bedard\Hexchess\Util;

testJson('san-parse', function ($t) {
    if ($t['error']) {
        expect(fn () => San::from($t['san']))->toThrow(\InvalidArgumentException::class);
    } else {
        $san = San::from($t['san']);
        expect($san->from)->toBe(Util::index($t['expect']['from']));
        expect($san->to)->toBe(Util::index($t['expect']['to']));
        expect($san->promotion)->toBe($t['expect']['promotion']);
    }
});
