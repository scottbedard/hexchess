import {
  index,
  graph,
  Hexchess
} from '../pkg/index'

let output: number[] = []

for (let i = 0; i < 91; i++) {
  const hexchess = new Hexchess()
  hexchess.board[i] = 'N'
  const result = hexchess.movesFromUnsafe(i)

  output.push(result.length)
}

console.log(output)