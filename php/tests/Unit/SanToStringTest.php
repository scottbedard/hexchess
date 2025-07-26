<?php

use Bedard\Hexchess\San;

testJson('san-to-string', function ($t) {
    $san = new San($t['san']['from'], $t['san']['to'], $t['san']['promotion']);

    expect((string) $san)->toBe($t['expect']);
});
