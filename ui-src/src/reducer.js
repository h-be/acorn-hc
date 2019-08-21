/*
This file is the top level reducer.
Import all the reducers from each feature here

"reducers" are the things that contain key application logic
of how ACTIONS affect STATE
*/

import { combineReducers } from 'redux'

import goals from './goals/reducer'
import edges from './edges/reducer'
import goalForm from './goal-form/reducer'
import selection from './selection/reducer'
import hover from './hover/reducer'
import keyboard from './keyboard/reducer'
import screensize from './screensize/reducer'
// import anotherone from './another/path'

// combine reducers from each feature to create the top-level reducer
export default combineReducers({
  goals, // goals: goals,
  edges, // edges: edges,
  ui: combineReducers({
    goalForm,
    selection,
    hover,
    keyboard,
    screensize
  }) // ,
  // anotherone: anotherone
})
