<?php

use Bedard\Hexchess\Hexchess;

testYaml('apply-moves-unsafe', function ($spec) {
    if ($spec['error']) {
        expect(fn () => Hexchess::parse($spec['hexchess'])->applyMoveUnsafe($spec['moves']))->toThrow(\InvalidArgumentException::class);
    } else {
        $hexchess = Hexchess::parse($spec['hexchess'])->applyMoveUnsafe($spec['moves']);

        if ($spec['expected']) {
            expect((string) $hexchess, $spec['description'])->toBe($spec['expected']);
        }
    }
});
