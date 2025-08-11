<?php

use Bedard\Hexchess\San;

testYaml('stringify-san', function ($spec) {
    $san = new San(
        $spec['san']['from'],
        $spec['san']['to'],
        $spec['san']['promotion']
    );

    expect((string) $san)->toBe($spec['expected']);
});
