<?php

use Bedard\Hexchess\Board;

describe('positions', function () {
    $data = yaml('positions');

    foreach ($data as $index => $position) {
        test($position, function () use ($index, $position) {
            expect(Board::position($index))->toBe($position);
            expect(Board::index($position))->toBe($index);
        });
    }
});
