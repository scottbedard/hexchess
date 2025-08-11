<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\San;

testYaml('parse-san', function ($spec) {
    if ($spec['error']) {
        expect(fn () => San::from($spec['san']))->toThrow(\InvalidArgumentException::class);
    } else {
        $san = San::from($spec['san']);
        expect($san->from)->toBe(Board::index($spec['expected']['from']));
        expect($san->to)->toBe(Board::index($spec['expected']['to']));
        expect($san->promotion)->toBe($spec['expected']['promotion']);
    }
});
