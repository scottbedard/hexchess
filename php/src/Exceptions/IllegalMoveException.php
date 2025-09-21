<?php

namespace Bedard\Hexchess\Exceptions;

use Exception;

class IllegalMoveException extends Exception
{
  public string $san;

  public function __construct(string $san)
  {
    $this->san = $san;

    parent::__construct("Illegal move: {$san}");
  }
}