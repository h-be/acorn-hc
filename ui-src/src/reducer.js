/*
This file is the top level reducer.
Import all the reducers from each feature here

"reducers" are the things that contain key application logic
of how ACTIONS affect STATE
*/

import { combineReducers } from 'redux'

import goals from './goals/reducer'
import goalCreation from './goal-creation/reducer'
// import anotherone from './another/path'

// combine reducers from each feature to create the top-level reducer
export default combineReducers({
  goals: goals, // ,
  ui: combineReducers({
    goalCreation
  }) // ,
  // anotherone: anotherone
})
