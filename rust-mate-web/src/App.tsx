import { useState, useEffect } from "react";
import Board from "./Board/Board";
import { INITIAL_FEN } from "./Board/constants";
import Logs from "./Board/Logs";
import { AIResponse, TBoard, TColor } from "./Board/types";
import { get } from "./http-common";
import Controls from "./Board/Controls";

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
    sideToMove: TColor.White,
    winner: "-",
    movesHistory: [],
    zobrit: 0,
  });

  const [loading, setLoading] = useState(true);
  const [lastAIResponse, setLastAIResponse] = useState<
    AIResponse | undefined
  >();
  //   const [timeIsUp, setTimeIsUp] = useState(false);

  const fetchBoard = () => {
    get<TBoard>("/board", (board) => {
      setBoard(board);
      setLoading(false);
    });
  };

  useEffect(() => {
    fetchBoard();
  }, []);

  if (loading) {
    return (
      <div
        className="modal-background"
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <h1 style={{ color: "#fff", fontWeight: "bold" }}>RustMate</h1>
      </div>
    );
  }

  return (
    <div id="content-wrapper">
      {/* <Timer sideToMove={board.sideToMove} timeRunning={board.winner === '-' && !timeIsUp} setTimeIsUp={setTimeIsUp} /> */}
      <Board
        board={board}
        setBoard={setBoard}
        setLastAIResponse={setLastAIResponse}
      />
      <div style={{display: "flex", flexDirection: "column", height: "100%", rowGap: "1rem"}}>
        <Logs board={board} lastAIMoveLogs={lastAIResponse} />
        <Controls setBoard={setBoard} setLastAIResponse={setLastAIResponse}/>
      </div>
      <span id="credits" style={{position: "absolute", top: "24px", left: "24px"}}>Still in development. Made by <a href="https://github.com/thiago-buarqque" target="_blank">Thiago Buarque</a>.</span>
    </div>
  );
}

export default App;
