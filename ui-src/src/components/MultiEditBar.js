import React, { useState } from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import StatusPicker from './StatusPicker'
import StatusIcon from './StatusIcon'

import {
    updateGoal
} from '../goals/actions'

function MultiEditBar({ selectedGoals, updateGoal }) {
    const defaultViews = {
        status: false,
        squirrels: false
    }
    const [viewsOpen, setViews] = useState(defaultViews)

    const updateGoalsStatuses = (status) => {
        selectedGoals.forEach(goal => {
            updateGoal({
                content: goal.content,
                user_hash: goal.user_hash,
                unix_timestamp: Date.now(),
                hierarchy: goal.hierarchy,
                status
            }, goal.address)
        })
    }

    return (
        <div className='multi_edit_bar'>
            <div className='multi_edit_bar_item' onClick={() => setViews({ ...defaultViews, status: !viewsOpen.status })}>
                <StatusIcon size='medium' hideTooltip status={selectedGoals[0].status} />
                {viewsOpen.status && <StatusPicker statusClicked={updateGoalsStatuses} />}
            </div>
        </div>
    )
}

MultiEditBar.propTypes = {
    selectedGoals: PropTypes.arrayOf(PropTypes.shape({
        content: PropTypes.string.isRequired,
        user_hash: PropTypes.string.isRequired,
        unix_timestamp: PropTypes.number.isRequired,
        hierarchy: PropTypes.string.isRequired,
        status: PropTypes.string.isRequired,
        address: PropTypes.string
    })).isRequired,
    updateGoal: PropTypes.func.isRequired
}

function mapStateToProps(state) {
    return {
        selectedGoals: state.ui.selection.selectedGoals.map(address => state.goals[address])
    }
}

function mapDispatchToProps(dispatch) {
    return {
        updateGoal: (goal, address) => {
            return dispatch(updateGoal.create({ address, goal }))
        }
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(MultiEditBar)