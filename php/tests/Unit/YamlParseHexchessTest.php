<?php

use Bedard\Hexchess\Hexchess;

testYaml('parse-hexchess', function ($spec) {
    if ($spec['error']) {
        expect(fn () => Hexchess::parse($spec['hexchess']))->toThrow(\InvalidArgumentException::class);
        return;
    }

    $hexchess = Hexchess::parse($spec['hexchess']);

    expect($hexchess->board)->toEqual($spec['expected']);
});
