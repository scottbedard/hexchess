<?php

namespace Bedard\Hexchess\Pieces;

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;
use Bedard\Hexchess\San;

class StraightLinePiece
{
    public static function moves(
        Hexchess $hexchess,
        int $from,
        string $color,
        array $directions
    ): array {
        $result = [];

        foreach ($directions as $direction) {
            $path = Board::walk($hexchess, $from, $direction, $color);

            foreach ($path as $to) {
                $result[] = new San($from, $to);
            }
        }

        return $result;
    }
}
