<?php

use Bedard\Hexchess\Hexchess;

testJson('hexchess-apply', function ($spec) {
    if ($spec['error']) {
        expect(fn () => Hexchess::parse($spec['from'])->apply($spec['sequence']))->toThrow(\InvalidArgumentException::class);
    } else {
        $hexchess = Hexchess::parse($spec['from'])->apply($spec['sequence']);

        if ($spec['to']) {
            expect((string) $hexchess, $spec['description'])->toBe($spec['to']);
        }
    }
});
