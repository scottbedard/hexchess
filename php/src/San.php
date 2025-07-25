<?php

namespace Bedard\Hexchess;

use Bedard\Hexchess\Constants;

class San
{
    /** san from position */
    public int $from;

    /** san promotion value */
    public ?string $promotion;

    /** san to position */
    public int $to;

    public function __construct(
        string|int $from,
        string|int $to,
        ?string $promotion = null
    ) {
        $this->from = Constants::index($from);
        $this->to = Constants::index($to);
        $this->promotion = $promotion;
    }

    public static function from(string $source)
    {
        // from
        $from = array_values(array_filter(Constants::POSITIONS, fn ($position) => str_starts_with($source, $position)))[0] ?? null;

        if (!$from) {
            throw new \InvalidArgumentException("invalid san from: {$from}");
        }

        // to
        $tail = substr($source, strlen($from));
        $to = array_values(array_filter(Constants::POSITIONS, fn ($position) => str_starts_with($tail, $position)))[0] ?? null;

        if (!$to) {
            throw new \InvalidArgumentException("invalid san to: {$to}");
        }

        if ($from === $to) {
            throw new \InvalidArgumentException("invalid san: from and to are the same");
        }

        // promotion
        $promotion = null;

        if (strlen($source) > strlen($from) + strlen($to)) {
            $last = substr($source, -1);

            if (in_array($last, ['n', 'r', 'b', 'q'])) {
                if (self::isPromotionPosition($to)) {
                    $promotion = $last;
                } else {
                    throw new \InvalidArgumentException("invalid san promotion: {$last}");
                }
            } else {
                throw new \InvalidArgumentException("invalid san promotion: {$last}");
            }
        }

        if (strlen($from) + strlen($to) + ($promotion !== null ? 1 : 0) !== strlen($source)) {
            throw new \InvalidArgumentException("invalid san: {$source}");
        }

        return new self($from, $to, $promotion);
    }

    private static function isPromotionPosition(string|int $position): bool
    {
        return in_array(Constants::index($position), [
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
            90, // l1,
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
}
