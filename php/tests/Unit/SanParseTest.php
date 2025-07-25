<?php

use Bedard\Hexchess\Constants;
use Bedard\Hexchess\San;

$data = array_map(fn ($t) => [
    $t->description,
    $t->san,
    $t->error,
    $t->expect,
], json('san-parse'));

test('san parse', function ($desc, $san, $error, $expected) {
    if ($error) {
        expect(fn () => San::from($san))->toThrow(\InvalidArgumentException::class);
    } else {
        $san = San::from($san);
        expect($san->from)->toBe(Constants::index($expected->from));
        expect($san->to)->toBe(Constants::index($expected->to));
        expect($san->promotion)->toBe($expected->promotion);
    }
})->with($data);
