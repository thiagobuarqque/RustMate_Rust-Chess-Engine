import React from "react";
import { TBoard, TColor, TPiece, TPieceType } from "./types";
import { BISHOP_PROMOTION, BISHOP_PROMOTION_CAPTURE, COLUMNS, EMPTY_FEN, KNIGHT_PROMOTION, KNIGHT_PROMOTION_CAPTURE, QUEEN_PROMOTION, QUEEN_PROMOTION_CAPTURE, ROOK_PROMOTION, ROOK_PROMOTION_CAPTURE } from "./constants";
import BoardPiece, { PIECE_ICONS } from "./BoardPiece";

interface IProps {
    board: TBoard;
    column: number;
    onClickPiece: (piece: TPiece) => void;
	onMovePiece: (position: number) => void;
    piece: TPiece | undefined
    position: number;
    row: number;
    selectedPiecePosition: number | undefined;
	onPromotionSelection?: (promotionOption: number) => void;
}

const Cell: React.FC<IProps> = ({
    board,
    column,
    onClickPiece,
	onMovePiece,
    piece,
    position,
    row,
    selectedPiecePosition,
	onPromotionSelection
}) => {
	const getCellClasses = () => {
		let classes = "cell";
	
		if (selectedPiecePosition === position) {
			classes += " selected"
		} 

		return classes;
	}

	return (
		<div
			key={column}
			className={getCellClasses()}
			data-pos={position}
			onClick={() => onMovePiece(position)}
		>
			{/* <span className="cell-index">{position}</span> */}
			{column === 7 && <span className={`row-index ${(row) % 2 === 0 ? "white" : ""}`}>{row + 1}</span>}
			{row === 0 && <span className={`column-index ${(column ) % 2 !== 0 ? "white" : ""}`}>{COLUMNS[7 - column]}</span>}
			{piece && piece.fen !== EMPTY_FEN ? (
				<BoardPiece
					blackKingInCheck={board.blackKingInCheck}
					boardPiece={piece}
					onClick={onClickPiece}
					whiteKingInCheck={board.whiteKingInCheck}
				/>
			) : (
				<div className="move-dot"></div>
			)}

			{
				onPromotionSelection &&
				<PromotionOptions 
					black={piece?.color == TColor.Black} 
					capture={piece?.type !== TPieceType.Empty}
					onPromotionSelection={onPromotionSelection} />
			}
		</div>
	);
};

const BLACK_PROMOTION_OPTIONS_FENS = ['q', 'b', 'n', 'r'];
const WHITE_PROMOTION_OPTIONS_FENS = ['Q', 'B', 'N', 'R'];

const PROMOTION_OPTIONS = [QUEEN_PROMOTION, BISHOP_PROMOTION, KNIGHT_PROMOTION, ROOK_PROMOTION];
const PROMOTION_CAPTURE_OPTIONS = [QUEEN_PROMOTION_CAPTURE, BISHOP_PROMOTION_CAPTURE, KNIGHT_PROMOTION_CAPTURE, ROOK_PROMOTION_CAPTURE];

const PromotionOptions: React.FC<{
	black: boolean,
	capture: boolean,
	onPromotionSelection: (promotionOption: number) => void;
}> = ({black, capture, onPromotionSelection}) => {

	const options = black ? BLACK_PROMOTION_OPTIONS_FENS : WHITE_PROMOTION_OPTIONS_FENS;

	return <> 
		<div className="modal-background"></div>
		<div id="promotion-options">
			{
				options.map((fen, i) => 
					<button
						key={fen}
						className="promotion-option"
						onClick={() => {
							onPromotionSelection(capture ? PROMOTION_CAPTURE_OPTIONS[i] : PROMOTION_OPTIONS[i])
						}}
						>
						<img className="piece" src={PIECE_ICONS[fen]} alt={fen} />
					</button>
				)
			}
		</div>
	</>
}

// const getInCheckClass = (blackKingInCheck: boolean, piece: number, whiteKingInCheck: boolean) => {
//     if(piece === (TColor.Black | TPieceType.King) && blackKingInCheck) {
//       return 'in-check';
//     }
  
//     if(piece === (TColor.White | TPieceType.King) && whiteKingInCheck) {
//       return 'in-check';
//     }
  
//     return '';
//   }

export default Cell;
