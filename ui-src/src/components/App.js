import React from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import GoalForm from './GoalForm'
import Help from './Help'
import MultiEditBar from './MultiEditBar'
import HoverOverlay from './HoverOverlay'

function App(props) {
  const { hasSelection, hasHover } = props
  return (
    <div>
      <GoalForm />
      <Help />
      {hasSelection && <MultiEditBar />}
      {hasHover && <HoverOverlay />}
    </div>
  )
}

App.propTypes = {
  hasSelection: PropTypes.bool.isRequired, // whether or not there are Goals selected
  hasHover: PropTypes.bool.isRequired, // whether or not a Goal is hovered over
}

function mapStateToProps(state) {
  return {
    // map from an array type (the selectedGoals) to a simple boolean type
    hasSelection: state.ui.selection.selectedGoals.length > 0,
    hasHover: !!state.ui.hover.hoveredGoal // converts (null or an address) to a boolean
  }
}

export default connect(mapStateToProps)(App)
