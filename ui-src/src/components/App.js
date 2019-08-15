import React from 'react'

import GoalForm from './GoalForm'

export default function App() {
  // The following instructions are just placeholder
  return (
    <div>
      <p className='instructions'>Hold down 'g' and click anywhere to start creating a Goal</p>
      <p className='instructions'>Press 'Esc' to close the Goal creator</p>
      <GoalForm />
    </div>
  )
}
