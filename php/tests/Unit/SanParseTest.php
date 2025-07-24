<?php

use Bedard\Hexchess\San;

describe('san parse', function () {
    $tests = json('san-parse');

    foreach ($tests as $t) {
        test($t['description'], function () use ($t) {
            // ...
        });
    }

    // foreach ($cases as $case) {
    //     $description = $case['description'] ?? 'no description';
    //     $sanString = $case['san'];
    //     $expectError = $case['error'] ?? false;
    //     $expect = $case['expect'] ?? null;

    //     if ($expectError) {
    //         try {
    //             $san = new San($sanString);
    //             $thrown = false;
    //         } catch (\Throwable $e) {
    //             $thrown = true;
    //         }
    //         expect($thrown)->toBeTrue("Expected error for case: $description");
    //     } else {
    //         // If San expects separate from/to/promotion, parse them
    //         $from = $expect['from'];
    //         $to = $expect['to'];
    //         $promotion = $expect['promotion'] ?? null;
    //         $san = new San($from, $to, $promotion);

    //         expect($san->from)->toBe(is_string($from) ? array_search($from, \Bedard\Hexchess\Constants::POSITIONS, true) : $from);
    //         expect($san->to)->toBe(is_string($to) ? array_search($to, \Bedard\Hexchess\Constants::POSITIONS, true) : $to);
    //         expect($san->promotion)->toBe($promotion);
    //     }
    // }
});
