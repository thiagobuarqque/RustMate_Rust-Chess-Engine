import React from 'react'

import './timer.scss';

const Timer = () => {
  return (
    <div id='timer'>
      <span id="black-time" className='text-secondary'>10:00</span>
      <hr></hr>
      <span id="white-time" className='text-primary-darker-1'>10:00</span>
    </div>
  )
}

export default Timer