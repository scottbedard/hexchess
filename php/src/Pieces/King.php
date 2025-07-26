<?php

namespace Bedard\Hexchess\Pieces;

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Hexchess;
use Bedard\Hexchess\San;

class King
{
    public static function moves(Hexchess $hexchess, int $from, string $color): array
    {
        $moves = [];

        for ($i = 0; $i < 12; $i++) {
            $to = Board::step($from, $i);

            if ($hexchess->board[$to] === null || Board::color($hexchess->board[$to]) !== $color) {
                $moves[] = new San($from, $to);
            }
        }

        return $moves;
    }
}
