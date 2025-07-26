<?php

namespace Bedard\Hexchess;

use Bedard\Hexchess\Board;
use Bedard\Hexchess\Constants;
use Bedard\Hexchess\Pieces\King;
use Bedard\Hexchess\Pieces\Knight;
use Bedard\Hexchess\Pieces\Pawn;
use Bedard\Hexchess\Pieces\StraightLinePiece;

class Hexchess
{
    /** @var array<int> */
    public array $board = [];

    /** @var 'w' | 'b' */
    public string $turn = 'w';

    /** @var int|null */
    public ?int $ep = null;

    /** @var int */
    public int $halfmove = 0;

    /** @var int */
    public int $fullmove = 1;

    /**
     * Create a new Hexchess instance.
     */
    public function __construct(string $fen = Constants::EMPTY_POSITION)
    {
        if (!$fen) {
            throw new \InvalidArgumentException('FEN string is required');
        }

        $parts = array_values(array_filter(array_map('trim', explode(' ', $fen)), fn ($str) => $str !== ''));
        $board = $parts[0] ?? null;
        $turn = $parts[1] ?? 'w';
        $ep = $parts[2] ?? '-';
        $halfmove = $parts[3] ?? '0';
        $fullmove = $parts[4] ?? '1';

        $this->board = $this->parseBoard($board);

        if ($turn === 'w' || $turn === 'b') {
            $this->turn = $turn;
        } else {
            throw new \InvalidArgumentException('Invalid turn color: ' . $turn);
        }

        if ($ep === '-') {
            $this->ep = null;
        } elseif ($this->isEnPassantPosition($ep)) {
            $this->ep = Board::index($ep);
        } else {
            throw new \InvalidArgumentException('Invalid en passant position: ' . $ep);
        }

        if (is_string($halfmove) && preg_match('/^-?\d+$/', $halfmove)) {
            $this->halfmove = max(0, (int) $halfmove);
        } else {
            throw new \InvalidArgumentException('Invalid halfmove: ' . $halfmove);
        }

        if (is_string($fullmove) && preg_match('/^-?\d+$/', $fullmove) && (int) $fullmove > 0) {
            $this->fullmove = (int) $fullmove;
        } else {
            throw new \InvalidArgumentException('Invalid fullmove: ' . $fullmove);
        }
    }

    /** format hexchess as fen */
    public function __toString(): string
    {
        $board = $this->stringifyBoard();
        $turn = $this->turn;
        $ep = $this->ep === null ? '-' : Board::position($this->ep);
        $halfmove = $this->halfmove;
        $fullmove = $this->fullmove;
        return "{$board} {$turn} {$ep} {$halfmove} {$fullmove}";
    }

    /** apply move, regardless of turn or legality */
    public function applyMoveUnsafe(San $san)
    {
        $piece = $this->board[$san->from];

        if ($piece === null) {
            throw new \InvalidArgumentException('Cannot apply move from empty position: ' . $san->from);
        }

        // update halfmove
        if ($this->board[$san->to] !== null || $piece === 'p' || $piece === 'P') {
            $this->halfmove = 0;
        } else {
            $this->halfmove += 1;
        }

        // update fullmove and turn color
        $color = Board::color($piece);

        if ($color === 'b') {
            $this->fullmove += 1;
            $this->turn = 'w';
        } else {
            $this->turn = 'b';
        }

        // set from positions
        $this->board[$san->from] = null;

        // set to position
        if ($san->promotion) {
            $this->board[$san->to] = $color === 'b' ? $san->promotion : strtoupper($san->promotion);
        } else {
            $this->board[$san->to] = $piece;
        }

        // clear captured en passant
        if ($san->to === $this->ep) {
            $captured = $piece === 'p'
                ? Board::step($san->to, 0)
                : ($piece === 'P'
                    ? Board::step($san->to, 6)
                    : null);

            if ($captured !== null) {
                $this->board[$captured] = null;
            }
        }

        // set en passsant
        if ($piece === 'p') {
            if ($san->from === 17 && $san->to === 38) {
                $this->ep = 27;
            } // c7 -> c5, c6
            elseif ($san->from === 18 && $san->to === 39) {
                $this->ep = 28;
            } // d7 -> d5, d6
            elseif ($san->from === 19 && $san->to === 40) {
                $this->ep = 29;
            } // e7 -> e5, e6
            elseif ($san->from === 20 && $san->to === 41) {
                $this->ep = 30;
            } // f7 -> f5, f6
            elseif ($san->from === 21 && $san->to === 42) {
                $this->ep = 31;
            } // g7 -> g5, g6
            elseif ($san->from === 22 && $san->to === 43) {
                $this->ep = 32;
            } // h7 -> h5, h6
            elseif ($san->from === 23 && $san->to === 44) {
                $this->ep = 33;
            } // i7 -> i5, i6
            elseif ($san->from === 24 && $san->to === 45) {
                $this->ep = 34;
            } // k7 -> k5, k6
            else {
                $this->ep = null;
            }
        } elseif ($piece === 'P') {
            if ($san->from === 71 && $san->to === 49) {
                $this->ep = 60;
            } // c2 -> c4, c3
            elseif ($san->from === 61 && $san->to === 39) {
                $this->ep = 50;
            } // d3 -> d5, d4
            elseif ($san->from === 51 && $san->to === 29) {
                $this->ep = 40;
            } // e4 -> e6, e5
            elseif ($san->from === 41 && $san->to === 20) {
                $this->ep = 30;
            } // f5 -> f7, f6
            elseif ($san->from === 53 && $san->to === 31) {
                $this->ep = 42;
            } // g4 -> g6, g5
            elseif ($san->from === 65 && $san->to === 43) {
                $this->ep = 54;
            } // h3 -> h5, h4
            elseif ($san->from === 77 && $san->to === 55) {
                $this->ep = 66;
            } // i2 -> i4, i3
            elseif ($san->from === 89 && $san->to === 67) {
                $this->ep = 78;
            } // k1 -> k3, k2
            else {
                $this->ep = null;
            }
        } else {
            $this->ep = null;
        }

        return $this;
    }

    /** find king by color */
    public function findKing(string $color): int | null
    {
        $king = $color === 'b' ? 'k' : 'K';

        for ($i = 0; $i < 91; $i++) {
            if ($this->board[$i] === $king) {
                return $i;
            }
        }

        return null;
    }

    /** get a piece from a position */
    public function get(string $position): string | null
    {
        try {
            $i = Board::index($position);

            return $this->board[$i] ?? null;
        } catch (\InvalidArgumentException $e) {
            return null;
        }
    }

    /** get positions occupied by a color */
    public function getColor(string $color): array
    {
        $result = [];

        for ($i = 0; $i < 91; $i++) {
            $piece = $this->board[$i];

            if ($piece && Board::color($piece) === $color) {
                $result[] = $i;
            }
        }

        return $result;
    }

    /**
     * Create a new Hexchess instance with the initial position.
     */
    public static function init(): self
    {
        return new self(Constants::INITIAL_POSITION);
    }

    /** test if position is legal en passant */
    private function isEnPassantPosition(string $position): bool
    {
        return in_array($position, [
            'b6',
            'c6',
            'd6',
            'e6',
            'f6',
            'g6',
            'h6',
            'i6',
            'k6',
            'b2',
            'c3',
            'd4',
            'e5',
            'f6',
            'g5',
            'h4',
            'i3',
            'k2',
        ]);
    }

    /** get legal moves from a position */
    public function movesFrom(int $from): array
    {
        $result = $this->movesFromUnsafe($from);

        return $result;
    }

    /** get moves from a position, regardless of turn or legality */
    public function movesFromUnsafe(int $from): array
    {
        $i = is_string($from) ? Board::index($from) : $from;

        $piece = $this->board[$i];

        if ($piece === null) {
            return [];
        }

        $color = Board::color($piece);

        switch ($piece) {
            case 'b':
            case 'B':
                return StraightLinePiece::moves($this, $i, $color, [1, 3, 5, 7, 9, 11]);
            case 'k':
            case 'K':
                return King::moves($this, $i, $color);
            case 'n':
            case 'N':
                return Knight::moves($this, $i, $color);
            case 'p':
            case 'P':
                return Pawn::moves($this, $i, $color);
            case 'q':
            case 'Q':
                return StraightLinePiece::moves($this, $i, $color, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
            case 'r':
            case 'R':
                return StraightLinePiece::moves($this, $i, $color, [0, 2, 4, 6, 8, 10]);
        }

        return [];
    }

    /**
     * Parse a FEN string into a Hexchess instance.
     */
    public static function parse(string $fen): self
    {
        return new self($fen);
    }

    /**
     * Parse the board from a FEN string.
     *
     * @return array<string|null>
     */
    private function parseBoard(string $source): array
    {
        $source = trim($source);
        $board = array_fill(0, 91, null);
        $black = false;
        $white = false;
        $j = 0;

        for ($i = 0; $i < strlen($source); $i++) {
            $current = $source[$i];

            switch ($current) {
                case '1':
                    $next = $source[$i + 1] ?? null;

                    switch ($next) {
                        case '0':
                            $j += 10;
                            $i++;
                            break;
                        case '1':
                            $j += 11;
                            $i++;
                            break;
                        default:
                            $j++;
                            break;
                    }
                    break;
                case '2':
                case '3':
                case '4':
                case '5':
                case '6':
                case '7':
                case '8':
                case '9':
                    $j += (int)$current;
                    break;

                case 'K':
                    if ($white) {
                        throw new \InvalidArgumentException('Multiple white kings');
                    }
                    $white = true;
                    $board[$j] = 'K';
                    $j++;
                    break;

                case 'k':
                    if ($black) {
                        throw new \InvalidArgumentException('Multiple black kings');
                    }
                    $black = true;
                    $board[$j] = 'k';
                    $j++;
                    break;
                case 'b':
                case 'B':
                case 'n':
                case 'N':
                case 'p':
                case 'P':
                case 'q':
                case 'Q':
                case 'r':
                case 'R':
                    $board[$j] = $current;
                    $j++;
                    break;
                case '/':
                    break;
                default:
                    throw new \InvalidArgumentException("Invalid board character: $current");
            }
        }

        if ($j !== 91) {
            throw new \InvalidArgumentException("Invalid board length: $j");
        }

        return $board;
    }

    /** format the board section of a fen */
    private function stringifyBoard(): string
    {
        $blank = 0;
        $index = 0;
        $result = '';

        foreach ($this->board as $piece) {
            if ($piece === null) {
                $blank += 1;
            } else {
                if ($blank > 0) {
                    $result .= $blank;
                    $blank = 0;
                }

                $result .= $piece;
            }

            if (in_array($index, [0, 3, 8, 15, 24, 35, 46, 57, 68, 79])) {
                if ($blank > 0) {
                    $result .= $blank;
                }

                $result .= '/';
                $blank = 0;
            }

            $index += 1;
        }

        if ($blank > 0) {
            $result .= $blank;
        }

        return $result;
    }
}
