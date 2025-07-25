<?php

use Bedard\Hexchess\Hexchess;

$data = array_map(fn ($t) => [
    $t->description,
    $t->fen,
    $t->error,
    $t->result,
], json('hexchess-parse'));

test('san parse', function ($desc, $fen, $error, $expected) {
    if ($error) {
        expect(fn () => Hexchess::parse($fen))->toThrow(\InvalidArgumentException::class);
    } else {
        $hexchess = Hexchess::parse($fen);
        expect($hexchess->board)->toBe($expected->board);
        expect($hexchess->turn)->toBe($expected->turn);
        expect($hexchess->ep)->toBe($expected->ep);
        expect($hexchess->halfmove)->toBe($expected->halfmove);
        expect($hexchess->fullmove)->toBe($expected->fullmove);
    }
})->with($data);
