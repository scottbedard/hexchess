<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Constants;

foreach (Constants::POSITIONS as $index => $position) {
    test("position {$position}", function () use ($index, $position) {
        expect(Board::position($index))->toBe($position);
    });
}
