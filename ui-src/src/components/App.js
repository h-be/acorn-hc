import React from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import GoalForm from './GoalForm'
import Help from './Help'
import MultiEditBar from './MultiEditBar'

function App(props) {
  const { hasSelection } = props
  return (
    <div>
      <GoalForm />
      <Help />
      {hasSelection && <MultiEditBar />}
    </div>
  )
}

App.propTypes = {
  hasSelection: PropTypes.bool.isRequired, // whether or not there are Goals selected
}

function mapStateToProps(state) {
  return {
    // map from an array type (the selectedGoals) to a simple boolean type
    hasSelection: state.ui.selection.selectedGoals.length > 0
  }
}

export default connect(mapStateToProps)(App)
