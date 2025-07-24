<?php

namespace Bedard\Hexchess;

class San
{
    public int $from;

    public int|null $promotion;

    public int $to;

    /**
     * Create a new san instance
     *
     * @param int|string $from
     * @param int|string $to
     * @param int|null $promotion
     */
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
