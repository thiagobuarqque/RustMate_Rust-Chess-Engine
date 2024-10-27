import { useState, useEffect } from 'react';
import Board from './Board/Board'
import { INITIAL_FEN } from './Board/constants';
import Logs from './Board/Logs'
import Timer from './Board/Timer'
import { TBoard, TColor } from './Board/types'
import http, { postWithBody } from "./http-common";

function App() {
  const [board, setBoard] = useState<TBoard>({
		blackCaptures: [],
		blackKingInCheck: false,
		enPassant: 0,
		evaluation: 0,
		fen: INITIAL_FEN,
		whiteCaptures: [],
		pieces: [],
		whiteKingInCheck: false,
		siteToMove: TColor.White,
		winner: "-",
		zobrit: 0,
	});

  const [loading, setLoading] = useState(true);
  const [timeIsUp, setTimeIsUp] = useState(false);

  const fetchBoard = async () => {
		return http
			.get<TBoard>("/board")
			.then((response) => response.data)
			.then((data) => {
				setBoard(data);
        setLoading(false);
			});
	};

  useEffect(() => {
		fetchBoard();
	}, []);

  if (loading) {
    return <div className='modal-background' style={{display: 'flex', alignItems: 'center', justifyContent: 'center'}}>
      <h1 style={{color: '#fff', fontWeight: 'bold'}}>RustMate</h1>
    </div>
  }

  return (
    <div id="content-wrapper">
      <Timer sideToMove={board.siteToMove} timeRunning={board.winner === '-' && !timeIsUp} setTimeIsUp={setTimeIsUp} />
      <Board board={board} setBoard={setBoard}/>
      <Logs aiResponse={undefined} board={board} isWaitingForAI={false} />
    </div>
  )
}

export default App
