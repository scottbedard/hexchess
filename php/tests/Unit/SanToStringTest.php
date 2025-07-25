<?php

use Bedard\Hexchess\Constants;
use Bedard\Hexchess\San;

$data = array_map(fn ($t) => [
    $t->description,
    $t->san,
    $t->expect,
], json('san-to-string'));

test('san parse', function ($desc, $s, $expected) {
    $san = new San($s->from, $s->to, $s->promotion);

    expect((string) $san)->toBe($expected);
})->with($data);
