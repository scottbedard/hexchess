<?php

namespace Bedard\Hexchess;

use Bedard\Hexchess\Constants;

class Util
{
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
}
