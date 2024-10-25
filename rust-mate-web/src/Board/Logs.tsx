import React from 'react'
import { AIResponse, TBoard } from './types'

import './logs.scss';

interface IProps {
    aiResponse: AIResponse | undefined,
    board: TBoard,
    isWaitingForAI: boolean
}

const Logs: React.FC<IProps> = ({aiResponse, board, isWaitingForAI}) => {
  return (
    <div id='logs'>
        <ul id="moves">
          <li><button className='move-btn'>e5</button></li>
          <li><button className='move-btn selected'>♞xe5</button></li>
          <li><button className='move-btn'>♖e5</button></li>
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