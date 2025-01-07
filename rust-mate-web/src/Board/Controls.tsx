import https from '../http-common';
import { INITIAL_FEN } from './constants';
import './controls.scss';
import { AIResponse, TBoard } from './types';

interface IProps {
  setBoard: (board: TBoard) => void;
  setLastAIResponse: (aiREsponse: AIResponse | undefined) => void;
}

const Controls: React.FC<IProps> = ({setBoard, setLastAIResponse}) => {
  const newGame = () => {
      https
      .post<TBoard>("/board/load/fen", {fen: INITIAL_FEN})
      .then((response) => response.data)
      .then((data) => {
        setBoard(data);
        setLastAIResponse(undefined)
      })
      .catch((err) => {
        console.error(err);
      });

  }
  return <div id="controls">
    <button id="new-game" onClick={newGame}>New Game</button>
  </div>
}

export default Controls;
