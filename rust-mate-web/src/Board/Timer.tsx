import React, { useCallback, useEffect, useRef, useState } from 'react'

import './timer.scss';
import { TEN_MINUTES_IN_SECONDS } from './constants';
import { TColor } from './types';

function formatSecondsToMinutes(seconds: number): string {
	const minutes = Math.floor(seconds / 60);
	const remainingSeconds = seconds % 60;
	return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
}

interface IProps {
  sideToMove: TColor,
  timeRunning: boolean,
  setTimeIsUp: (value: boolean) => void
}

const Timer: React.FC<IProps> = ({ sideToMove, timeRunning, setTimeIsUp }) => {
  const [timers, setTimers] = useState({
    [TColor.Black]: TEN_MINUTES_IN_SECONDS,
    [TColor.White]: TEN_MINUTES_IN_SECONDS,
  });

  const timeoutRefs = useRef({
    [TColor.Black]: -1 as number | undefined,
    [TColor.White]: -1 as number | undefined,
  });

  const handleTimeout = useCallback((color: TColor, newValue: number) => {
    timeoutRefs.current[color] = setTimeout(() => {
      setTimers((prevTimers) => ({ ...prevTimers, [color]: newValue }));
    }, 1000);
  }, []);

  const decrement = useCallback(() => {
    if (!timeRunning) return;

    const color = sideToMove;
    const newValue = timers[color] - 1;

    if (newValue === -1) {
      clearTimeout(timeoutRefs.current[color]);
      setTimeIsUp(true);
    } else {
      handleTimeout(color, newValue);
    }
  }, [sideToMove, timeRunning, timers, handleTimeout, setTimeIsUp]);

  useEffect(() => {
    decrement();
  }, [decrement]);

  useEffect(() => {
    clearTimeout(timeoutRefs.current[TColor.Black]);
    clearTimeout(timeoutRefs.current[TColor.White]);
    decrement();
  }, [sideToMove, decrement]);

  return (
    <div id="timer">
      <span id="black-time" className={sideToMove === TColor.Black ? 'text-primary-darker-1' : 'text-secondary'}>
        {formatSecondsToMinutes(timers[TColor.Black])}
      </span>
      <hr />
      <span id="white-time" className={sideToMove === TColor.White ? 'text-primary-darker-1' : 'text-secondary'}>
        {formatSecondsToMinutes(timers[TColor.White])}
      </span>
    </div>
  );
};

export default Timer;
