/*
  This file is ENTRY POINT to the entire web application.
  Only code that gets called at some point from this file (via imports or otherwise)
  will get executed in the browser window.
  Try to keep it clean and minimal in this file, and outsource aspects to more
  modular code in separate files, imported here and called.
*/

// Library Imports
import React from 'react'
import ReactDOM from 'react-dom'
import { Provider } from 'react-redux'
import { createStore, applyMiddleware, compose } from 'redux'
import { connect } from '@holochain/hc-web-client'
import { holochainMiddleware } from '@holochain/hc-redux-middleware'

// Local Imports
import acorn from './reducer'
import render from './drawing'
import { createGoal } from './goals/actions'
import GoalFormContainer from './components/GoalFormContainer'

const defaultHolochainPort = '8888'

// this url should use the same port set up by the Holochain Conductor
const websocketUrl = `ws://localhost:${defaultHolochainPort}`
// attempts to form a websocket (two way messages) connection to a running
// Holochain Conductor
const hcWc = connect({ url: websocketUrl })

// holochainMiddleware takes in the hc-web-client websocket connection
// and uses it to facilitate the calls to Holochain
const middleware = [holochainMiddleware(hcWc)]

// This enables the redux-devtools browser extension
// which gives really awesome debugging for apps that use redux
const composeEnhancers = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose

// acorn is the top-level reducer. the second argument is custom Holochain middleware
let store = createStore(acorn, /* preloadedState, */ composeEnhancers(
  applyMiddleware(...middleware)
))

/*
  store.subscribe(cb)
  store.getState()
  store.dispatch(action)
*/

store.dispatch(createGoal.create({ entry: {
  content: "Test title //o= <- spider",
  user_hash: "Boop",
  unix_timestamp: 412,
  complete: false,
  certain: true,
  small: true, }}))

store.dispatch(createGoal.create({ entry: {
  content: "Another test! 😇",
  user_hash: "Boop",
  unix_timestamp: 413,
  complete: false,
  certain: true,
  small: true, }}))

// store.dispatch(createGoal.create({ entry: { content: "Sample Title! 😛" }}))
// store.dispatch(createGoal.create({ entry: { content: "Another one! 😇" }}))


/* SETUP THE REACT CONTAINER, WHERE DOM ELEMENTS WILL BE RENDERED */
const reactContainer = document.createElement('div')
reactContainer.className = 'react'


/* SETUP THE CANVAS SPACE, WHERE MANY VISUAL non-dom ELEMENTS WILL BE RENDERED */
const canvas = document.createElement('canvas')
canvas.width = document.body.clientWidth
canvas.height = document.body.clientHeight


document.body.appendChild(canvas)
document.body.appendChild(reactContainer)

// whenever the STATE in the STORE changes, re-render the state data to the canvas
store.subscribe(() => {
  render(store, canvas)
})

// Do an initial draw of the view
render(store, canvas)

// By passing the `store` in as a wrapper around our React component
// we make the state available throughout it
ReactDOM.render(
  <Provider store={store}>
    <GoalFormContainer />
  </Provider>,
  reactContainer
)
