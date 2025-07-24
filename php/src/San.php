<?php

namespace Bedard\Hexchess;

class San
{
    public int $from;

    public int|null $promotion;

    public int $to;

    public function __construct(
        int|string $from,
        int|string $to,
        $promotion = null
    ) {
        // ...
    }
}
