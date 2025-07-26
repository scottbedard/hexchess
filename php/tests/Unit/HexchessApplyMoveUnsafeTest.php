<?php

use Bedard\Hexchess\Hexchess;
use Bedard\Hexchess\San;

testJson('hexchess-apply-move-unsafe', function ($spec) {
    if ($spec['error']) {
        expect(fn () => Hexchess::parse($spec['from'])->applyMoveUnsafe(San::from($spec['sequence'])))->toThrow(\InvalidArgumentException::class);
    } else {
        $hexchess = Hexchess::parse($spec['from'])->applyMoveUnsafe(San::from($spec['sequence']));

        if ($spec['to']) {
            expect((string) $hexchess, $spec['description'])->toBe($spec['to']);
        }
    }
});
