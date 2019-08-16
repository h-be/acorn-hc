import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import { createGoal } from '../goals/actions'
import { closeGoalCreator, updateContent } from '../goal-creation/actions'


// a React Component is defined as a class that extends the basic
// React Component class. Usually of the form
// class MyComponent extends Component
class GoalForm extends Component {
  constructor() {
    // calls the constructor of the `super` (parent) class,
    // which does its own initialization steps.
    // important, always call this.
    super()

    // always required to make sure that functions defined on the class
    // have the proper `this` scope attached to them.
    // its a weird oddity of JS and React,
    // don't question it, just do it.
    this.handleChange = this.handleChange.bind(this)
    this.handleKeyDown = this.handleKeyDown.bind(this)
    this.handleSubmit = this.handleSubmit.bind(this)
  }

  /*
    EVENT HANDLERS
  */
  handleChange(event) {
    this.props.updateContent(event.target.value)
  }
  handleKeyDown(event) {
    if (event.key === 'Enter') {
      event.preventDefault()
      this.handleSubmit()
    }
  }
  handleSubmit(event) {
    if (event) {
      // this is to prevent the page from refreshing
      // when the form is submitted, which is the
      // default behaviour
      event.preventDefault()
    }

    // dispatch the action to create a goal
    // with the contents from the form
    // inserted into it
    this.props.createGoal({
      goal: {
        content: this.state.goal_title,
        user_hash: "Boop",
        unix_timestamp: 413,
        complete: false,
        certain: false,
        small: false
      }
    })
    // reset the textarea value to empty
    this.props.updateContent('')
    this.props.closeGoalCreator()
  }

  /*
    RENDER FUNCTION
  */
  render() {
    // Contained within the `render` function of a React component
    // is the strict definition of what HTML should appear on screen
    // depending on the data that is given to the component

    const { content, isOpen, xLoc, yLoc } = this.props

    // use the xLoc and yLoc to position the form anywhere on the screen
    // using a position relative to its container

    // optionally render the form dependent on whether the `isOpen` prop
    // is true, else render nothing
    return (<div style={{ position:'absolute', top:`${yLoc}px`, left:`${xLoc}px`}}>
      {isOpen ? <form className='goal_form' onSubmit={this.handleSubmit}>
        <textarea
          placeholder='Add a title...'
          value={content}
          onChange={this.handleChange}
          onKeyDown={this.handleKeyDown}
          autoFocus
        />
      </form> : null}
    </div>)
  }
}

// Setting propTypes helps us keep track of which properties
// a custom component that we define expects to be passed,
// and helps us to understand it more quickly as a developer
// looking at the code after it was written
GoalForm.propTypes = {
  content: PropTypes.string.isRequired,
  isOpen: PropTypes.bool.isRequired,
  xLoc: PropTypes.number.isRequired,
  yLoc: PropTypes.number.isRequired,
  createGoal: PropTypes.func.isRequired,
  closeGoalCreator: PropTypes.func.isRequired
}

// https://react-redux.js.org/using-react-redux/connect-mapstate
// Designed to grab selective data off of a redux state tree in order
// to pass it to a component for rendering, as specific properties that
// that component expects
function mapStateToProps(state) {
  // all the state for this component is store under state->ui->goalCreation
  const { content, isOpen, xLoc, yLoc } = state.ui.goalCreation
  // the name of the expected proptypes is the same
  // as the name of the properties as stored in state
  return {
    content,
    isOpen,
    xLoc,
    yLoc
  }
}

// https://react-redux.js.org/using-react-redux/connect-mapdispatch
// Designed to pass functions into components which are already wrapped as
// action dispatchers for redux action types
function mapDispatchToProps(dispatch) {
  return {
    updateContent: (content) => {
      dispatch(updateContent(content))
    },
    createGoal: (goal) => {
      dispatch(createGoal.create(goal))
    },
    closeGoalCreator: () => {
      dispatch(closeGoalCreator())
    }
  }
}

// the `connect` function wraps the basic component into a component
// that receives the appropriate `props`.
export default connect(mapStateToProps, mapDispatchToProps)(GoalForm)
