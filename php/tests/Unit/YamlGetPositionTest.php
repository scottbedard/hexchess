<?php

use Bedard\Hexchess\Hexchess;

testYaml('get-position', function ($spec) {
  $hexchess = Hexchess::parse($spec['hexchess']);
  $actual = $hexchess->get($spec['position']);

  expect($actual)->toBe($spec['expected']);
});
