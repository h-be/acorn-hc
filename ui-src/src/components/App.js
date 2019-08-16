import React from 'react'

import GoalForm from './GoalForm'

export default function App() {
  // The following instructions are just placeholder
  return (
    <div>
      <p className='instructions'>Hold down 'g' and click anywhere to start creating a Goal</p>
      <p className='instructions'>Click on a node to select it</p>
      <p className='instructions'>With a node selected, hold down 'g' and click anywhere to create a connected (child) Goal</p>
      <p className='instructions'>Press 'Esc' to close the Goal creator and deselect Goals</p>
      <GoalForm />
    </div>
  )
}
