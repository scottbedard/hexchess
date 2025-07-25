<?php

namespace Bedard\Hexchess;

use Bedard\Hexchess\Constants;

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
            $this->ep = Constants::index($ep);
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
}
