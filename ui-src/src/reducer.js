/*
This file is the top level reducer.
Import all the reducers from each feature here

"reducers" are the things that contain key application logic
of how ACTIONS affect STATE
*/

import { combineReducers } from 'redux'

import goals from './goals/reducer'
import edges from './edges/reducer'
import goalCreation from './goal-creation/reducer'
import selection from './selection/reducer'
// import anotherone from './another/path'

// combine reducers from each feature to create the top-level reducer
export default combineReducers({
  goals, // goals: goals,
  edges, // edges: edges,
  ui: combineReducers({
    goalCreation,
    selection
  }) // ,
  // anotherone: anotherone
})
