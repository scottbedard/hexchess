<?php

namespace Bedard\Hexchess\Pieces;

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Constants;
use Bedard\Hexchess\Hexchess;
use Bedard\Hexchess\San;

class Knight
{
    public static function moves(Hexchess $hexchess, int $from, string $color): array
    {
        $moves = [];

        $graph = Constants::KNIGHT_GRAPH[$from];

        foreach ($graph as $to) {
            if ($hexchess->board[$to] === null || Board::color($hexchess->board[$to]) !== $color) {
                $moves[] = new San($from, $to);
            }
        }

        return $moves;
    }
}
