<?php

namespace Bedard\Hexchess;

use Bedard\Hexchess\Constants;
use Bedard\Hexchess\Enums\Color;

class Board
{
    /** get the color of a piece */
    public static function color(string $piece): string
    {
        return $piece === 'k' || $piece === 'q' || $piece === 'r' || $piece === 'b' || $piece === 'n' || $piece === 'p'
            ? 'b'
            : 'w';
    }

    /** normalize position to index */
    public static function index(int|string $position): int
    {
        if (is_int($position)) {
            return $position;
        }

        foreach (Constants::POSITIONS as $i => $p) {
            if ($position === $p) {
                return $i;
            }
        }

        throw new \InvalidArgumentException("invalid position: {$position}");
    }

    /** normalize index to position */
    public static function position(int|string $index): string
    {
        if (is_int($index)) {
            return Constants::POSITIONS[$index];
        }

        return Constants::POSITIONS[self::index($index)];
    }

    /** get the next position in a given direction */
    public static function step(int $from, int $direction): int | null
    {
        return Constants::GRAPH[$from][$direction];
    }

    /** walk along the hexboard graph */
    public static function walk(Hexchess $hexchess, int $from, int $direction, string $color): array
    {
        $path = [];
        $position = $from;

        while (true) {
            $next = self::step($position, $direction);

            if ($next === null) {
                return $path; // <- end of board
            }

            $position = $next;

            $piece = $hexchess->board[$position];

            if ($piece === null) {
                $path[] = $position; // <- unoccupied position
                continue;
            }

            if (self::color($piece) === $color) {
                return $path; // <- shop short of friendly piece
            }

            $path[] = $position; // <- and captury enemy piece

            return $path;
        }
    }
}
