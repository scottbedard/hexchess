<?php

namespace Bedard\Hexchess\Pieces;

use Bedard\Hexchess\Hexchess;
use Bedard\Hexchess\San;
use Bedard\Hexchess\Board;

class Pawn
{
    /** advance forward one position */
    public static function advance(Hexchess $hexchess, int $start, int $from, int $forward): ?San
    {
        // we don't need to verify the step exists, because pawns cannot exist
        // on the final rank without promoting. there will always be one more step.
        $to = Board::step($from, $forward);

        return $hexchess->board[$to] === null ? new San($start, $to) : null;
    }

    /** capture if possible */
    public static function capture(Hexchess $hexchess, int $from, int $direction, string $friendly): ?San
    {
        $to = Board::step($from, $direction);

        if (!is_int($to)) {
            return null;
        }

        $target = $hexchess->board[$to];

        if ($target) {
            if (Board::color($target) !== $friendly) {
                return new San($from, $to);
            }
        } elseif ($hexchess->ep === $to && $hexchess->turn === $friendly) {
            return new San($from, $to);
        }

        return null;
    }

    /** expand promotions */
    public static function expand(San $san, string $color): array
    {
        if (self::isPromotionPosition($san->to, $color)) {
            return [
                new San($san->from, $san->to, 'b'),
                new San($san->from, $san->to, 'n'),
                new San($san->from, $san->to, 'q'),
                new San($san->from, $san->to, 'r'),
            ];
        }

        return [$san];
    }

    /** test if position is a promotion position */
    public static function isPromotionPosition(int $position, string $color): bool
    {
        return $color === 'b'
            ? in_array($position, [
                80, // a1
                81, // b1
                82, // c1
                83, // d1
                84, // e1
                85, // f1
                86, // g1
                87, // h1
                88, // i1
                89, // k1
                90, // l1
            ])
            : in_array($position, [
                25, // a6
                16, // b7
                9, // c8
                4, // d9
                1, // e10
                0, // f11
                3, // g10
                8, // h9
                15, // i8
                24, // k7
                35, // l6
            ]);
    }

    /** check if position is a starting position */
    public static function isStartingPosition(int $position, string $color): bool
    {
        return $color === 'b'
            ? in_array($position, [
                16, // b7
                17, // c7
                18, // d7
                19, // e7
                20, // f7
                21, // g7
                22, // h7
                23, // i7
                24, // k7
            ])
            : in_array($position, [
                81, // b1
                71, // c2
                61, // d3
                51, // e4
                41, // f5
                53, // g4
                65, // h3
                77, // i2
                89, // k1
            ]);
    }

    /** get all moves unsafe */
    public static function moves(Hexchess $hexchess, int $from, string $color): array
    {
        $moves = [];

        $forward = $color === 'w' ? 0 : 6;
        $portside = $color === 'w' ? 10 : 4;
        $starboard = $color === 'w' ? 2 : 8;

        // advance forward one position
        $advance1 = self::advance($hexchess, $from, $from, $forward);

        if ($advance1) {
            array_push($moves, ...self::expand($advance1, $color));

            // if starting position, advance forward another position
            if (self::isStartingPosition($from, $color)) {
                $advance2 = self::advance($hexchess, $from, $advance1->to, $forward);

                if ($advance2) {
                    array_push($moves, ...self::expand($advance2, $color));
                }
            }
        }

        // capture portside
        $capturePortside = self::capture($hexchess, $from, $portside, $color);

        if ($capturePortside) {
            array_push($moves, ...self::expand($capturePortside, $color));
        }

        // capture starboard
        $captureStarboard = self::capture($hexchess, $from, $starboard, $color);

        if ($captureStarboard) {
            array_push($moves, ...self::expand($captureStarboard, $color));
        }

        return $moves;
    }
}
