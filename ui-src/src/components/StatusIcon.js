import React from 'react'
import PropTypes from 'prop-types'

function StatusIcon({ status, selected, onClick }) {
    const classList = `status_color ${status} ${selected ? 'has_status' : ''} ${onClick ? 'can_click' : ''}`
    return (
        <div className={classList} onClick={() => onClick(status)}>
            <div className='hidden_tooltip'>{status}</div>
        </div>
    )
}

StatusIcon.propTypes = {
    status: PropTypes.string.isRequired,
    selected: PropTypes.bool,
    onClick: PropTypes.func
}

export default StatusIcon