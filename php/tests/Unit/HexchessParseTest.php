<?php

use Bedard\Hexchess\Hexchess;

testJson('hexchess-parse', function ($t) {
    if ($t['error']) {
        expect(fn () => Hexchess::parse($t['fen']))->toThrow(\InvalidArgumentException::class);
    } else {
        $hexchess = Hexchess::parse($t['fen']);
        expect($hexchess->board)->toBe($t['result']['board']);
        expect($hexchess->turn)->toBe($t['result']['turn']);
        expect($hexchess->ep)->toBe($t['result']['ep']);
        expect($hexchess->halfmove)->toBe($t['result']['halfmove']);
        expect($hexchess->fullmove)->toBe($t['result']['fullmove']);
    }
});
