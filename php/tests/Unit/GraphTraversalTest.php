<?php

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;

describe('graph traversal', function () {
    $hexchess = new Hexchess();
    $specs = json('graph-traversal');

    foreach ($specs as $spec) {
        $from = Board::index($spec['from']);

        for ($direction = 0; $direction < 12; $direction++) {
            $result = $spec['results'][$direction];

            test($spec['from'].' -> '.$direction, function () use ($hexchess, $from, $direction, $result) {
                $positions = array_map(fn ($n) => Board::position($n), Board::walk($hexchess, $from, $direction, 'w'));

                expect($positions)->toEqual($result);
            });
        }
    }
});
