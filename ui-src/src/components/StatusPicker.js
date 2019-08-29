import React, { useState } from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import StatusIcon from './StatusIcon'

import {
    updateGoal,
} from '../goals/actions'

function StatusPicker({ goal, goalAddress, updateGoal }) {
    const update = (status) => {
        updateGoal({
            content: goal.content,
            user_hash: goal.user_hash,
            unix_timestamp: Date.now(),
            hierarchy: goal.hierarchy,
            status
        }, goalAddress)
    }

    const statuses = [
        'Uncertain',
        'Incomplete',
        'Complete',
        'InReview'
    ]

    return (
        <div className='status_picker vertical_action_overlay'>
            {statuses.map(status => (
                <StatusIcon status={status} selected={goal.status === status} onClick={update} />
            ))}
        </div>
    )
}

StatusPicker.propTypes = {
    goal: PropTypes.shape({
        content: PropTypes.string.isRequired,
        user_hash: PropTypes.string.isRequired,
        unix_timestamp: PropTypes.number.isRequired,
        hierarchy: PropTypes.string.isRequired,
        status: PropTypes.string.isRequired
    }).isRequired,
    goalAddress: PropTypes.string.isRequired,
    updateGoal: PropTypes.func.isRequired
}

function mapStateToProps(state) {
    const goalAddress = state.ui.goalForm.editAddress
    return {
        goalAddress,
        goal: state.goals[goalAddress]
    }
}

function mapDispatchToProps(dispatch) {
    return {
        updateGoal: (goal, address) => {
            return dispatch(updateGoal.create({ address, goal }))
        }
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(StatusPicker)