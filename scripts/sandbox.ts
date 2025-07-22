import {
  index,
  graph,
  Hexchess
} from '../js/index'

let output: any[] = []

for (let i = 0; i < 91; i++) {
  const hexchess = new Hexchess()
  hexchess.board[i] = 'N'
  const result = hexchess.movesFromUnsafe(i)

  output.push(
    result.map(r => r.to).join(', ')
  )
}

console.log(output)