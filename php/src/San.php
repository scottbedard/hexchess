<?php

namespace Bedard\Hexchess;

class San
{
    /** san from position */
    public int $from;

    /** san promotion value */
    public int|null $promotion;

    /** san to position */
    public int $to;

    public function __construct(
        int|string $from,
        int|string $to,
        $promotion = null
    ) {
        $this->from = is_string($from)
            ? array_search($from, Constants::POSITIONS, true)
            : $from;

        $this->to = is_string($to)
            ? array_search($to, Constants::POSITIONS, true)
            : $to;

        $this->promotion = $promotion;
    }
}
