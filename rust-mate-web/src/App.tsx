import Board from './Board/Board'
import Logs from './Board/Logs'
import Timer from './Board/Timer'
import { TColor } from './Board/types'

function App() {
  return (
    <div id="content-wrapper">
      <Timer />
      <Board/>
      <Logs aiResponse={undefined} board={{
        blackCaptures: [],
        blackKingInCheck: false,
        enPassant: 0,
        evaluation: 0,
        fen: '',
        pieces: [],
        whiteCaptures: [],
        whiteKingInCheck: false,
        siteToMove: TColor.Black,
        winner: '-',
        zobrit: 0
      }} isWaitingForAI={false} />
    </div>
  )
}

export default App
