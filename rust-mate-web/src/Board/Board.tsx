import React, { useEffect, useState } from "react";

import { PIECE_ICONS } from "./BoardPiece";
import { AIResponse, TBoard, TMove, TColor, TPieceType, TPiece } from "./types";

import captureAudio from "../assets/sound/capture.mp3";
import moveAudio from "../assets/sound/move-self.mp3";

import http from "../http-common";

import "./board.scss";
import { CAPTURE, INITIAL_FEN } from "./constants";

import Cell from "./Cell";

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

const Board = () => {
	const [selectedPiecePos, setSelectedPiecePos] = useState<number>(INVALID_POSITION);
	const [lastMovePos, setLastMovePos] = useState<number>(INVALID_POSITION);
	const [isWaitingForAI, setIsWaitingForAI] = useState(false);
	const [lastAIResponse, setLastAIResponse] = useState<AIResponse>();
	// const [promotionOption, setPromotionOption] = useState<number>(0);
	const [promotionPosition, setPromotionPosition] = useState<number>(INVALID_POSITION);
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

	const onPieceSelect = (piece: TPiece) => {
		if (isWaitingForAI) {
			return;
		}

		if (board.siteToMove !== piece.color) {
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
		document.querySelectorAll('.capture-receptor').forEach((el) => {
			el.classList.remove('capture-receptor')
		});

		document.querySelectorAll('.empty-receptor').forEach((el) => {
			el.classList.remove('empty-receptor')
		});

		document.querySelectorAll('button.piece-button.disabled').forEach((el) => {
			el.classList.remove('disabled')
		});
	}

	const showPieceAvailableMoves = (position: number) => {
		// if (isWaitingForAI) {
		// 	return;
		// }

		board.pieces[position].moves.forEach((move) => {
			const to_position = move.to

			const capturedPiece = board.pieces[to_position];

			const className = capturedPiece.type !== TPieceType.Empty ? "capture-receptor" : "empty-receptor";

			const cell = document.querySelector(`.cell[data-pos='${to_position}']`) as HTMLDivElement;

			cell.classList.add(className);

			const cellPiece = document.querySelector(
				`.cell[data-pos='${to_position}'] button.piece-button`
			) as HTMLDivElement;

			cellPiece?.classList.add("disabled");
		});
	};

	const onMovePiece = (cell: HTMLDivElement, cellPosition: number) => {
		if (selectedPiecePos === INVALID_POSITION || cellPosition == selectedPiecePos) {
      		return;
		}

		let moves = board.pieces[selectedPiecePos].moves;

		moves = moves.filter((move) => move.to === cellPosition)

		if (!moves.length) {
			console.log("Could not find move");

			return;
		}

		// TODO: Add the option to choose the promotion
		if (moves.length > 1 && promotionPosition === INVALID_POSITION) {
			setPromotionPosition(moves[0].to);
		} else if (promotionPosition === INVALID_POSITION) {
			movePiece(moves[0]);
		}

	};

	function movePiece(move: TMove) {
		setSelectedPiecePos(INVALID_POSITION);

		const cellPiece = document.querySelector(
			`.cell[data-pos='${selectedPiecePos}'] button.piece-button.disabled`
		) as HTMLDivElement;

		cellPiece?.classList.remove("disabled");

		playMoveAudio((move.flags & CAPTURE) !== 0);

		hideAllAvailableMoves();

		makeMove(move);
		setLastMovePos(move.from);
	}

	const movePromotionPawn = (promotionOption: number) => {
		// setPromotionPosition(promotionOption);

		let moves = board.pieces[selectedPiecePos].moves;

		const move = moves.filter((move) => move.to === promotionPosition && move.flags === promotionOption);

		if (move.length != 1) {
			alert("Error when promoting pawn")

			return;
		}

		movePiece(move[0]);
		setPromotionPosition(INVALID_POSITION);
	}

	const fetchBoard = async () => {
		return http
			.get<TBoard>("/board")
			.then((response) => response.data)
			.then((data) => {
				setBoard(data);
			});
	};

	const makeMove = (move: TMove) => {
		http.post<TBoard>("/board/move/piece", move)
			.then((response) => response.data)
			.then((data) => {
				setBoard(data);
			})
			.catch((err) => {
				console.error(err);
			});
	};

	const loadFEN = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();

		const inputFen: HTMLInputElement | null = document.getElementById("input-fen") as HTMLInputElement;

		if (!inputFen) return;

		let fen = INITIAL_FEN;

		if (inputFen.value.trim() !== "") {
			fen = inputFen.value.trim();
		}

		http.post<TBoard>("/board/load/fen", {
			fen,
		})
			.then((response) => response.data)
			.then((data) => {
				setBoard(data);

				setLastAIResponse(undefined);
			});
	};

	useEffect(() => {
		fetchBoard();
	}, []);

	return (
		<>
			<div id="board">
				<div id="winner-modal" className={`modal-background ${board.winner === "-" ? 'opacity-0 no-point-events' : ''}`}>
					<div id="content">
						<h1>{board.winner}{board.winner !== 'Draw' ? ' won!' : ''}</h1>
					</div>
				</div>
				<div id="white-captures" className="captures">
					{board.whiteCaptures.map((piece_fen, i) => (
						<img key={i} className="captured_piece" src={PIECE_ICONS[piece_fen]} alt={piece_fen} />
					))}
				</div>
				<div id="black-captures" className="captures">
					{board.blackCaptures.map((piece_fen, i) => (
						<img key={i} className="captured_piece" src={PIECE_ICONS[piece_fen]} alt={piece_fen} />
					))}
				</div>
				{[7, 6, 5, 4, 3, 2, 1, 0].map((row) => (
					<div key={row} className="row">
						{[0, 1, 2, 3, 4, 5, 6, 7].map((column) => {
              				const position = (row * 8) + (7 - column);

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
									onPromotionSelection={promotionPosition === position ? movePromotionPawn : undefined}
								/>
							);
						}).reverse()}
					</div>
				))}
			</div>
		</>
	);
};

export default Board;
