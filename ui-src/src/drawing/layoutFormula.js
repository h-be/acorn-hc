import { goalWidth, goalHeight } from './dimensions'


function findAParent(edges, address) {
  // highly oversimplified
  const edge = edges.find(e => e.child_address === address)
  return edge ? edge.parent_address : null
}


function mapGoalToHierarchy(goal, edges) {
  // setup and run a recursive call to find depth/hierarchy in the graph
  // TODO: make this more resilient, highly oversimplified
  let hierarchy = 1

  function checkHierarchy(address) {
    const parent = findAParent(edges, address)

    if (parent) {
      // increment the hierarchy var each time an additional parent
      // is found
      hierarchy++
      checkHierarchy(parent)
    }
  }

  checkHierarchy(goal.address)

  // console.log(hierarchy)

  return {
    hierarchy,
    goal
  }
}


function mapHierarchyToPosition({ goal, hierarchy }, withHierarchies, screenWidth) {
  const verticalOffset = 10
  const verticalSpacing = 100
  const horizontalSpacing = 20

  // FIXME: this needs to be related to the display pixel ratio or something
  const RETINA_HACK_HALFSCREEN = 4

  const sameTier = withHierarchies.filter((wH) => wH.hierarchy === hierarchy)
  const indexInTier = sameTier.map((wH) => wH.goal.content).indexOf(goal.content)

  const horizontalHalfScreen = screenWidth / RETINA_HACK_HALFSCREEN
  const halfGoalWidth = goalWidth / 2
  const totalWidth = goalWidth + horizontalSpacing
  const x = horizontalHalfScreen + (indexInTier * totalWidth) - ((sameTier.length - 1) * totalWidth)/2 - halfGoalWidth

  // default position is a function of the hierarchical status of the goal
  const y = verticalOffset + hierarchy * (goalHeight + verticalSpacing)

  return {
    x,
    y
  }
}

export default function layoutFormula(screenWidth, goals, edges) {
  // assign hierarchical statuses to things
  const withHierarchies = goals.map(g => mapGoalToHierarchy(g, edges))

  // use positions in the hierarchy to determine coordinates
  return withHierarchies.map(wH => mapHierarchyToPosition(wH, withHierarchies, screenWidth))
}
