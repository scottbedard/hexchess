<?php

namespace Bedard\Hexchess;

use Bedard\Hexchess\Board;
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
        $this->from = Board::index($from);
        $this->to = Board::index($to);
        $this->promotion = $promotion;
    }

    /**
     * Parse san from string
     */
    public static function from(string $source): self
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

    /** check if position is a promotion position */
    private static function isPromotionPosition(string|int $position): bool
    {
        return in_array(Board::position($position), [
            'a1',
            'b1',
            'c1',
            'd1',
            'e1',
            'f1',
            'g1',
            'h1',
            'i1',
            'k1',
            'l1',
            'a6',
            'b7',
            'c8',
            'd9',
            'e10',
            'f11',
            'g10',
            'h9',
            'i8',
            'k7',
            'l6',
        ]);
    }

    /** convert san to string */
    public function __toString(): string
    {
        return Board::position($this->from) . Board::position($this->to) . ($this->promotion ?? '');
    }
}
