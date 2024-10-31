import React from 'react'
import { TBoard, TMove } from './types'

import './logs.scss';

interface IProps {
    board: TBoard,
}

const getMoves = (moves: TMove[]) => {
  const elements = []

  let i;
  for(i = 0; i < moves.length; i++) {
    if(i + 1 == moves.length) {
      break
    }

    elements.push(
      <li key={i}>
        <span className='index text-primary'>{(i / 2) + 1}.</span>
        <div>
          <button className='move-btn'>{moves[i].notation}</button>
          <button className={'move-btn ' + (i + 1 == moves.length - 1 ? 'selected' : '')}>{moves[i+1].notation}</button>
        </div>
      </li>
    )

    i++;
  }

  if (moves.length % 2 !== 0) {
    elements.push(
      <li key={i}>
        <span className='index text-primary'>{(i / 2) + 1}.</span>
        <div>
          <button className='move-btn selected'>{moves[i].notation}</button>
        </div>
      </li>
    )
  }

  return elements;
}

const Logs: React.FC<IProps> = ({board}) => {
  return (
    <div id='logs'>
        <h3>Moves History</h3>
        <ul id="moves">
          {
            getMoves(board.movesHistory)
          }

          {/* <li>
              <span className='index text-primary'>1.</span>
            <div>
              <button className='move-btn'>e5</button>
              <button className='move-btn'>e5</button>
            </div>
          </li>
          <li>
              <span className='index text-primary'>2.</span>
            <div>
              <button className='move-btn selected'>♞xe5</button>
              <button className='move-btn selected'>♞xe5</button>
            </div>
          </li>
          <li>
              <span className='index text-primary'>3.</span>
            <div>
              <button className='move-btn'>♖e5</button>
              <button className='move-btn'>♖e5</button>
            </div>
          </li> */}
        </ul>

        <div id="move-infos">
          <div className="move-info">
            <span className="move-info-label">Zobrist:</span>
            <span className="move-info-value">---</span>
          </div>
          <div className="move-info">
            <span className="move-info-label">Nodes searched:</span>
            <span className="move-info-value">---</span>
          </div>
          <div className="move-info">
            <span className="move-info-label">Time:</span>
            <span className="move-info-value">---</span>
          </div>
          <div className="move-info">
            <span className="move-info-label">Depth:</span>
            <span className="move-info-value">---</span>
          </div>
          <div className="move-info">
            <span className="move-info-label">Position evaluation:</span>
            <span className="move-info-value">---</span>
          </div>
        </div>
    </div>
  )
}

const getZobritBinary = (zobrit: number) => {
    let binary = zobrit.toString(2)
  
    while(binary.length < 64) {
      binary = "0" + binary;
    }
  
    return binary.slice(0, 16) + " " + binary.slice(16, 32) + "\n" + binary.slice(32, 48) + " " + binary.slice(48, 64);
  }

export const getBoardEvaluationMessage = (boardEvaluation: number, winner: "-" | "b" | "w" | "d") => {
    if(winner === 'd') {
        return "Draw";
    } else if(winner === 'b') {
        return "Black won!"
    } else if (winner === 'w') {
        return "White won!"
    }

    if(boardEvaluation === 0) {
      return "No one is winning";
    } else if(boardEvaluation < 0 && boardEvaluation >= -100) {
      return "Black is slightly better"
    } else if(boardEvaluation < -100) {
      return "Black is winning"
    } else if(boardEvaluation > 0 && boardEvaluation <= 100) {
      return "White is slightly better"
    } else {
      return "White is winning"
    }
  }

export default Logs