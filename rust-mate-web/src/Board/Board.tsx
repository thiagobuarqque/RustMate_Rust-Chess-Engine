import React, { useEffect, useState } from "react";

import { PIECE_ICONS } from "./BoardPiece";
import { AIResponse, TBoard, TMove, TPieceType, TPiece } from "./types";

import captureAudio from "../assets/sound/capture.mp3";
import moveAudio from "../assets/sound/move-self.mp3";

import http, { get, postWithBody } from "../http-common";

import "./board.scss";
import { CAPTURE, INITIAL_FEN, isCapture } from "./constants";

import Cell from "./Cell";
import Confetti from "../utils/confetti";

const playMoveAudio = (capture: boolean) => {
  let audio;

  if (capture) {
    audio = new Audio(captureAudio);
  } else {
    audio = new Audio(moveAudio);
  }
  audio.play();
};

const INVALID_POSITION = -1;

interface IProps {
  board: TBoard;
  setBoard: (board: TBoard) => void;
  setLastAIResponse: (lastAIResponse: AIResponse) => void;
}

const Board: React.FC<IProps> = ({ board, setBoard, setLastAIResponse }) => {
  const [selectedPiecePos, setSelectedPiecePos] =
    useState<number>(INVALID_POSITION);
  const [lastMovePos, setLastMovePos] = useState<number>(INVALID_POSITION);
  const [isWaitingForAI, setIsWaitingForAI] = useState(false);
  // const [promotionOption, setPromotionOption] = useState<number>(0);
  const [promotionPosition, setPromotionPosition] =
    useState<number>(INVALID_POSITION);

  const onPieceSelect = (piece: TPiece) => {
    if (isWaitingForAI) {
      return;
    }

    if (board.sideToMove !== piece.color) {
      alert("LOG: It's not your turn");
      // Play invalid move sound
      return;
    }

    hideAllAvailableMoves();

    if (selectedPiecePos === piece.position) {
      setSelectedPiecePos(INVALID_POSITION);

      // hideAllAvailableMoves();
    } else {
      // if (selectedPiecePos !== INVALID_POSITION) {
      // 	hideAllAvailableMoves();
      // }

      setSelectedPiecePos(piece.position);
      showPieceAvailableMoves(piece.position);
    }
  };

  const hideAllAvailableMoves = () => {
    document.querySelectorAll(".capture-receptor").forEach((el) => {
      el.classList.remove("capture-receptor");
    });

    document.querySelectorAll(".empty-receptor").forEach((el) => {
      el.classList.remove("empty-receptor");
    });

    document.querySelectorAll("button.piece-button.disabled").forEach((el) => {
      el.classList.remove("disabled");
    });
  };

  const showPieceAvailableMoves = (position: number) => {
    // if (isWaitingForAI) {
    // 	return;
    // }

    board.pieces[position].moves.forEach((move) => {
      const to_position = move.to;

      const capturedPiece = board.pieces[to_position];

      const className =
        capturedPiece.type !== TPieceType.Empty
          ? "capture-receptor"
          : "empty-receptor";

      const cell = document.querySelector(
        `.cell[data-pos='${to_position}']`,
      ) as HTMLDivElement;

      cell.classList.add(className);

      const cellPiece = document.querySelector(
        `.cell[data-pos='${to_position}'] button.piece-button`,
      ) as HTMLDivElement;

      cellPiece?.classList.add("disabled");
    });
  };

  const onMovePiece = (position: number) => {
    if (selectedPiecePos === INVALID_POSITION || position == selectedPiecePos) {
      return;
    }

    let moves = board.pieces[selectedPiecePos].moves;

    moves = moves.filter((move) => move.to === position);

    if (!moves.length) {
      console.log("Could not find move");

      return;
    }

    if (moves.length > 1 && promotionPosition === INVALID_POSITION) {
      setPromotionPosition(moves[0].to);
    } else if (promotionPosition === INVALID_POSITION) {
      movePiece(moves[0]);
    }
  };

  function movePiece(move: TMove) {
    setSelectedPiecePos(INVALID_POSITION);

    const cellPiece = document.querySelector(
      `.cell[data-pos='${selectedPiecePos}'] button.piece-button.disabled`,
    ) as HTMLDivElement;

    cellPiece?.classList.remove("disabled");

    playMoveAudio(isCapture(move.flags));

    hideAllAvailableMoves();

    makeMove(move);
    setLastMovePos(move.from);
  }

  const movePromotionPawn = (promotionOption: number) => {
    // setPromotionPosition(promotionOption);

    const moves = board.pieces[selectedPiecePos].moves;

    const move = moves.filter(
      (move) => move.to === promotionPosition && move.flags === promotionOption,
    );

    if (move.length != 1) {
      alert("Error when promoting pawn");

      return;
    }

    movePiece(move[0]);
    setPromotionPosition(INVALID_POSITION);
  };

  const getAiMove = () => {
    // setIsWaitingForAI(true);

    get<AIResponse>("/ai/move", (aiResponse: AIResponse) => {
      const move = aiResponse.aiMove;

      playMoveAudio(isCapture(move.flags));

      setLastAIResponse(aiResponse);

      get<TBoard>("/board", setBoard);
    });
  };

  const makeMove = (move: TMove) => {
    http
      .post<TBoard>("/board/move/piece", move)
      .then((response) => response.data)
      .then((data) => {
        setBoard(data);

        getAiMove();
      })
      .catch((err) => {
        console.error(err);
      });
  };

  const onLoadFEN = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const inputFen: HTMLInputElement | null = document.getElementById(
      "input-fen",
    ) as HTMLInputElement;

    if (!inputFen) return;

    let fen = INITIAL_FEN;

    if (inputFen.value.trim() !== "") {
      fen = inputFen.value.trim();
    }

    loadFen(fen);
  };

  const loadFen = (fen: string) => {
    postWithBody<TBoard>("/board/load/fen", { fen }, setBoard);
  };

  useEffect(() => {
    if (board.winner !== "-" && board.winner !== "Draw") {
      new Confetti();
    }
  }, [board.winner]);

  return (
    <>
      <div id="board">
        <div
          id="winner-modal"
          className={`${board.winner === "-" ? "" : "slide-top"}`}
        >
          <h1>
            {board.winner}
            {board.winner !== "Draw" ? " won!" : ""}
          </h1>
          <button
            className="play-as-btn primary"
            onClick={() => loadFen(INITIAL_FEN)}
          >
            Play Again
          </button>
          {/* <button className="play-as-btn primary">Play as White</button> */}
        </div>
        {/* <div id="winner-modal" className={`${board.winner === "-" ? 'opacity-0 no-point-events' : ''}`}>
				</div> */}
        <div id="white-captures" className="captures">
          {board.whiteCaptures.map((piece_fen, i) => (
            <img
              key={i}
              className="captured_piece"
              src={PIECE_ICONS[piece_fen]}
              alt={piece_fen}
            />
          ))}
        </div>
        <div id="black-captures" className="captures">
          {board.blackCaptures.map((piece_fen, i) => (
            <img
              key={i}
              className="captured_piece"
              src={PIECE_ICONS[piece_fen]}
              alt={piece_fen}
            />
          ))}
        </div>
        {[7, 6, 5, 4, 3, 2, 1, 0].map((row) => (
          <div key={row} className="row">
            {[0, 1, 2, 3, 4, 5, 6, 7]
              .map((column) => {
                const position = row * 8 + (7 - column);

                return (
                  <Cell
                    key={position}
                    board={board}
                    column={column}
                    onClickPiece={onPieceSelect}
                    onMovePiece={onMovePiece}
                    piece={board.pieces[position]}
                    position={position}
                    row={row}
                    selectedPiecePosition={selectedPiecePos}
                    onPromotionSelection={
                      promotionPosition === position
                        ? movePromotionPawn
                        : undefined
                    }
                  />
                );
              })
              .reverse()}
          </div>
        ))}
      </div>
    </>
  );
};

export default Board;
